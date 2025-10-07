pub use crate::prelude::*;

/// Status of the Task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskStatusStatus {
    #[serde(rename = "STATUS_INVALID")]
    StatusInvalid,
    #[serde(rename = "STATUS_CREATED")]
    StatusCreated,
    #[serde(rename = "STATUS_SCHEDULED_IN_MANAGER")]
    StatusScheduledInManager,
    #[serde(rename = "STATUS_SENT")]
    StatusSent,
    #[serde(rename = "STATUS_MACHINE_RECEIPT")]
    StatusMachineReceipt,
    #[serde(rename = "STATUS_ACK")]
    StatusAck,
    #[serde(rename = "STATUS_WILCO")]
    StatusWilco,
    #[serde(rename = "STATUS_EXECUTING")]
    StatusExecuting,
    #[serde(rename = "STATUS_WAITING_FOR_UPDATE")]
    StatusWaitingForUpdate,
    #[serde(rename = "STATUS_DONE_OK")]
    StatusDoneOk,
    #[serde(rename = "STATUS_DONE_NOT_OK")]
    StatusDoneNotOk,
    #[serde(rename = "STATUS_REPLACED")]
    StatusReplaced,
    #[serde(rename = "STATUS_CANCEL_REQUESTED")]
    StatusCancelRequested,
    #[serde(rename = "STATUS_COMPLETE_REQUESTED")]
    StatusCompleteRequested,
    #[serde(rename = "STATUS_VERSION_REJECTED")]
    StatusVersionRejected,
}
impl fmt::Display for TaskStatusStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::StatusInvalid => "STATUS_INVALID",
            Self::StatusCreated => "STATUS_CREATED",
            Self::StatusScheduledInManager => "STATUS_SCHEDULED_IN_MANAGER",
            Self::StatusSent => "STATUS_SENT",
            Self::StatusMachineReceipt => "STATUS_MACHINE_RECEIPT",
            Self::StatusAck => "STATUS_ACK",
            Self::StatusWilco => "STATUS_WILCO",
            Self::StatusExecuting => "STATUS_EXECUTING",
            Self::StatusWaitingForUpdate => "STATUS_WAITING_FOR_UPDATE",
            Self::StatusDoneOk => "STATUS_DONE_OK",
            Self::StatusDoneNotOk => "STATUS_DONE_NOT_OK",
            Self::StatusReplaced => "STATUS_REPLACED",
            Self::StatusCancelRequested => "STATUS_CANCEL_REQUESTED",
            Self::StatusCompleteRequested => "STATUS_COMPLETE_REQUESTED",
            Self::StatusVersionRejected => "STATUS_VERSION_REJECTED",
        };
        write!(f, "{}", s)
    }
}
