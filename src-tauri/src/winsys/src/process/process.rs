use crate::close_handle;
use crate::close_handle_option;
use crate::errors::Result;
use crate::process::pid::Pid;
use crate::types::wstr::WSTR;
use log::debug;
use thiserror::Error;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::WAIT_OBJECT_0;
use windows::Win32::Foundation::WAIT_TIMEOUT;
use windows::Win32::Security::DuplicateTokenEx;
use windows::Win32::Security::SecurityImpersonation;
use windows::Win32::Security::TOKEN_ALL_ACCESS;
use windows::Win32::Security::TOKEN_DUPLICATE;
use windows::Win32::Security::TOKEN_QUERY;
use windows::Win32::Security::TokenPrimary;
use windows::Win32::System::Threading::CREATE_PROCESS_LOGON_FLAGS;
use windows::Win32::System::Threading::CreateProcessW;
use windows::Win32::System::Threading::CreateProcessWithTokenW;
use windows::Win32::System::Threading::GetExitCodeProcess;
use windows::Win32::System::Threading::INFINITE;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::OpenProcessToken;
use windows::Win32::System::Threading::PROCESS_CREATION_FLAGS;
use windows::Win32::System::Threading::PROCESS_INFORMATION;
use windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION;
use windows::Win32::System::Threading::STARTF_USESTDHANDLES;
use windows::Win32::System::Threading::STARTUPINFOW;
use windows::Win32::System::Threading::WaitForInputIdle;
use windows::Win32::System::Threading::WaitForSingleObject;

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("进程已运行")]
    AlreadyRunningError,

    #[error("进程未运行")]
    NotRunningError,

    #[error("使用用户权限运行失败")]
    RunAsUserError,

    #[error("打开进程失败")]
    OpenProcess,

    #[error("打开进程令牌失败")]
    OpenProcessToken,

    #[error("创建进程失败")]
    CreateProcessWithTokenWError,

    #[error("复制令牌失败")]
    DuplicateTokenEx,

    #[error("创建进程失败")]
    CreateProcessWError,

    #[error("获取进程退出码失败")]
    GetExitCodeProcessError,

    #[error("等待超时")]
    WaitTimeoutError,

    #[error("等待失败")]
    WaitFailedError,
}

#[derive(Debug)]
pub struct Process {
    command_line: Option<String>,
    binherithandles: bool,
    creation_flags: PROCESS_CREATION_FLAGS,
    wait_for_idle: Option<u32>,
    startup_info: STARTUPINFOW,
    process_info: PROCESS_INFORMATION,
    use_token: bool,
    new_token: Option<HANDLE>,
    explorer_token: Option<HANDLE>,
    explorer_process: Option<HANDLE>,
}

impl Default for Process {
    fn default() -> Self {
        Self {
            command_line: None,
            binherithandles: false,
            creation_flags: PROCESS_CREATION_FLAGS(0),
            wait_for_idle: None,
            use_token: false,
            startup_info: Self::startupinfo(),
            process_info: PROCESS_INFORMATION::default(),
            new_token: None,
            explorer_token: None,
            explorer_process: None,
        }
    }
}

impl Process {
    pub fn try_create_as_user(command: &str) -> Result<Self> {
        if let Ok(p) = Self::create_as_user(command) {
            Ok(p)
        } else {
            Self::create(command)
        }
    }

    pub fn create(command: &str) -> Result<Self> {
        debug!("使用直接运行程序: {:?}", command);
        Self::new(command).run()
    }

    pub fn create_as_user(command: &str) -> Result<Self> {
        debug!("使用降权启动程序: {:?}", command);
        Self::new(command).run_as_user()
    }

    pub fn new<S: Into<String>>(command_line: S) -> Self {
        Self {
            command_line: Some(command_line.into()),
            ..Default::default()
        }
    }

    #[inline]
    fn startupinfo() -> STARTUPINFOW {
        let mut si = STARTUPINFOW::default();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as _;
        si
    }

    pub fn set_command<S: Into<String>>(mut self, command_line: S) -> Self {
        self.command_line = Some(command_line.into());
        self
    }

    pub fn set_wait_for_idle(mut self, milliseconds: u32) -> Self {
        self.wait_for_idle = Some(milliseconds);
        self
    }

    pub fn set_binherithandles(mut self, binherithandles: bool) -> Self {
        self.binherithandles = binherithandles;
        self
    }

    pub fn run(mut self) -> Result<Self> {
        if !self.process_info.hProcess.is_invalid() {
            return Err(ProcessError::AlreadyRunningError.into());
        }
        let mut wstr = WSTR::new(self.command_line.as_deref());
        unsafe {
            let _ = CreateProcessW(
                None,
                Some(wstr.to_pwstr()),
                None,
                None,
                self.binherithandles,
                self.creation_flags,
                None,
                None,
                &self.startup_info,
                &mut self.process_info,
            );
        };
        if let Some(timeout) = self.wait_for_idle {
            unsafe { WaitForInputIdle(self.process_info.hProcess, timeout) };
        }
        Ok(self)
    }

    pub fn run_as_user(mut self) -> Result<Self> {
        self.use_token = true;
        if !self.process_info.hProcess.is_invalid() {
            return Err(ProcessError::AlreadyRunningError.into());
        }
        let pid = Pid::get_explorer_pid()?;
        let explorer_process = unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION, false, pid)
                .map_err(|_| ProcessError::OpenProcess)?
        };
        let mut explorer_token = HANDLE::default();
        unsafe {
            OpenProcessToken(
                explorer_process,
                TOKEN_DUPLICATE | TOKEN_QUERY,
                &mut explorer_token,
            )
            .map_err(|_| ProcessError::OpenProcessToken)?
        };
        let mut new_token = HANDLE::default();
        unsafe {
            DuplicateTokenEx(
                explorer_token,
                TOKEN_ALL_ACCESS,
                None,
                SecurityImpersonation,
                TokenPrimary,
                &mut new_token,
            )
            .map_err(|_| ProcessError::DuplicateTokenEx)?
        };
        self.startup_info.hStdInput = explorer_process;
        self.startup_info.hStdOutput = explorer_process;
        self.startup_info.hStdError = explorer_process;
        self.startup_info.dwFlags |= STARTF_USESTDHANDLES;
        self.new_token = Some(new_token);
        self.explorer_process = Some(explorer_process);
        self.explorer_token = Some(explorer_token);
        let mut wstr = WSTR::new(self.command_line.as_deref());
        unsafe {
            CreateProcessWithTokenW(
                new_token,
                CREATE_PROCESS_LOGON_FLAGS(0),
                None,
                Some(wstr.to_pwstr()),
                self.creation_flags,
                None,
                None,
                &self.startup_info,
                &mut self.process_info,
            )
            .map_err(|_| ProcessError::CreateProcessWithTokenWError)?
        }
        if let Some(timeout) = self.wait_for_idle {
            unsafe { WaitForInputIdle(self.process_info.hProcess, timeout) };
        }
        Ok(self)
    }

    pub fn wait_for(&self, timeout: u32) -> Result<()> {
        let ret = unsafe { WaitForSingleObject(self.process_info.hProcess, timeout) };
        if ret == WAIT_OBJECT_0 {
            Ok(())
        } else if ret == WAIT_TIMEOUT {
            Err(ProcessError::WaitTimeoutError.into())
        } else {
            Err(ProcessError::WaitFailedError.into())
        }
    }

    pub fn wait(&self) -> Result<()> {
        self.wait_for(INFINITE)
    }

    pub fn get_exit_code(&self) -> Result<u32> {
        let mut exit_code: u32 = 0;
        unsafe {
            GetExitCodeProcess(self.process_info.hProcess, &mut exit_code)
                .map_err(|_| ProcessError::GetExitCodeProcessError)?
        };
        Ok(exit_code)
    }

    pub fn get_pid(&self) -> Pid {
        Pid::new(self.process_info.dwProcessId)
    }

    pub fn get_u32(&self) -> u32 {
        self.process_info.dwProcessId
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if !self.use_token {
            close_handle!(self.startup_info.hStdInput);
            close_handle!(self.startup_info.hStdOutput);
            close_handle!(self.startup_info.hStdError);
        }
        close_handle_option!(self.new_token);
        close_handle_option!(self.explorer_token);
        close_handle_option!(self.explorer_process);
        close_handle!(self.process_info.hThread);
        close_handle!(self.process_info.hProcess);
    }
}
