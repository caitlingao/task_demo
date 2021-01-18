// constants
pub const SINGULAR_PLURAL_THRESHOLD: i32 = 1;
pub const TASK_ID_INCREMENT_THRESHOLD: i32 = 1;
pub const ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds
pub const NANOSECOND_THRESHOLD: i64 = 1_000_000_000;

// file path
pub const TMP_DIR: &str = "tmp";
pub const DOWNLOAD_DIR: &str = "tmp/download";
pub const USER_FILE: &str = ".todo-config.json";
pub const TASKS_FILE: &str = "tmp/tasks.json";
pub const CACHE_FILE: &str = "tmp/cache.json";

// message
// signup
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
pub const MESSAGE_SIGNUP_FAILED: &str = "Signup failed";

// login logout
pub const LOGIN_SUCCESS: &str = "Login success.";
pub const LOGIN_FAILED: &str = "User does not exist or wrong password.";
pub const ASK_FOR_LOGIN: &str = "Please login first.";
pub const LOGOUT_SUCCESS: &str = "Logout success.";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

// file
pub const UNABLE_TO_READ_FILE: &str = "Unable to read file.";
pub const UNABLE_WRITE_TO_FILE: &str = "Unable write to file.";
pub const FILE_NOT_EXIST: &str = "File does not exist.";
pub const GET_FILE_DATA_WRONG: &str = "Get data from file wrong.";
pub const ASK_FOR_JSON_FILE: &str = "the file format must be json.";
pub const IMPORT_FILE_SUFFIX: &str = ".json";

// task
pub const TASK_DOES_NOT_EXIST: &str = "Task does not exist.";
pub const NO_TASK: &str = "There is not task.";
