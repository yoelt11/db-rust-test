// @generated automatically by Diesel CLI.

diesel::table! {
    keypoints (keypoint_id) {
        keypoint_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    objects (object_id) {
        object_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    poses (pose_id) {
        pose_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    room_object (room_id, object_id) {
        room_id -> Integer,
        object_id -> Integer,
    }
}

diesel::table! {
    rooms (room_id) {
        room_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    tier1_activities (tier1_id) {
        tier1_id -> Integer,
        tier1 -> Text,
    }
}

diesel::table! {
    tier1_activity_objects (object_id, tier1_id) {
        object_id -> Integer,
        tier1_id -> Integer,
    }
}

diesel::table! {
    tier1_activity_poses (pose_id, tier1_id) {
        pose_id -> Integer,
        tier1_id -> Integer,
    }
}

diesel::table! {
    tier1_activity_rooms (room_id, tier1_id) {
        room_id -> Integer,
        tier1_id -> Integer,
    }
}

diesel::table! {
    tier1activity_tier2activity (tier1_id, tier2_id) {
        tier1_id -> Integer,
        tier2_id -> Integer,
    }
}

diesel::table! {
    tier2_objects_keypoints (tier2_id, object_id, keypoint_id) {
        tier2_id -> Integer,
        object_id -> Integer,
        keypoint_id -> Integer,
    }
}

diesel::table! {
    tier2activities (tier2_id) {
        tier2_id -> Integer,
        tier2 -> Text,
    }
}

diesel::joinable!(room_object -> objects (object_id));
diesel::joinable!(room_object -> rooms (room_id));
diesel::joinable!(tier1_activity_objects -> objects (object_id));
diesel::joinable!(tier1_activity_poses -> poses (pose_id));
diesel::joinable!(tier1_activity_rooms -> rooms (room_id));
diesel::joinable!(tier1activity_tier2activity -> tier2activities (tier2_id));
diesel::joinable!(tier2_objects_keypoints -> keypoints (keypoint_id));
diesel::joinable!(tier2_objects_keypoints -> objects (object_id));
diesel::joinable!(tier2_objects_keypoints -> tier2activities (tier2_id));

diesel::allow_tables_to_appear_in_same_query!(
    keypoints,
    objects,
    poses,
    room_object,
    rooms,
    tier1_activities,
    tier1_activity_objects,
    tier1_activity_poses,
    tier1_activity_rooms,
    tier1activity_tier2activity,
    tier2_objects_keypoints,
    tier2activities,
);
