pub use crate::prelude::*;

/// Sensor Field Of View closely resembling fov.proto SensorFieldOfView.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldOfView {
    /// The Id for one instance of a FieldOfView, persisted across multiple updates to provide continuity during
    /// smoothing. This is relevant for sensors where the dwell schedule is on the order of
    /// milliseconds, making multiple FOVs a requirement for proper display of search beams.
    #[serde(rename = "fovId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fov_id: Option<i64>,
    /// The Id of the mount the sensor is on.
    #[serde(rename = "mountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_id: Option<String>,
    /// The field of view the sensor projected onto the ground.
    #[serde(rename = "projectedFrustum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_frustum: Option<ProjectedFrustum>,
    /// Center ray of the frustum projected onto the ground.
    #[serde(rename = "projectedCenterRay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_center_ray: Option<Position>,
    /// The origin and direction of the center ray for this sensor relative to the ENU frame. A ray which is aligned with
    /// the positive X axis in the sensor frame will be transformed into the ray along the sensor direction in the ENU
    /// frame when transformed by the quaternion contained in this pose.
    #[serde(rename = "centerRayPose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_ray_pose: Option<Pose>,
    /// Horizontal field of view in radians.
    #[serde(rename = "horizontalFov")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_fov: Option<f64>,
    /// Vertical field of view in radians.
    #[serde(rename = "verticalFov")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_fov: Option<f64>,
    /// Sensor range in meters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f64>,
    /// The mode that this sensor is currently in, used to display for context in the UI. Some sensors can emit multiple
    /// sensor field of views with different modes, for example a radar can simultaneously search broadly and perform
    /// tighter bounded tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<FieldOfViewMode>,
}
