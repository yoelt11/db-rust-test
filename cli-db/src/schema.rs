// @generated automatically by Diesel CLI.

diesel::table! {
    keypoint_hits (kph_id) {
        kph_id -> Integer,
        object_id -> Integer,
        keypoint_id -> Integer,
    }
}

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
    tier1activities (tier1_id) {
        tier1_id -> Integer,
        tier1 -> Text,
    }
}

diesel::table! {
    tier2_activity_poses (pose_id, tier2_id) {
        pose_id -> Integer,
        tier2_id -> Integer,
    }
}

diesel::table! {
    tier2_tier1_kph (tier2_id, tier1_id, kph_id) {
        tier2_id -> Integer,
        tier1_id -> Integer,
        kph_id -> Integer,
    }
}

diesel::table! {
    tier2activities (tier2_id) {
        tier2_id -> Integer,
        tier2 -> Text,
    }
}

diesel::joinable!(keypoint_hits -> keypoints (keypoint_id));
diesel::joinable!(keypoint_hits -> objects (object_id));
diesel::joinable!(room_object -> objects (object_id));
diesel::joinable!(room_object -> rooms (room_id));
diesel::joinable!(tier1_activity_objects -> objects (object_id));
diesel::joinable!(tier1_activity_objects -> tier1activities (tier1_id));
diesel::joinable!(tier1_activity_poses -> poses (pose_id));
diesel::joinable!(tier1_activity_poses -> tier1activities (tier1_id));
diesel::joinable!(tier1_activity_rooms -> rooms (room_id));
diesel::joinable!(tier1_activity_rooms -> tier1activities (tier1_id));
diesel::joinable!(tier2_activity_poses -> poses (pose_id));
diesel::joinable!(tier2_activity_poses -> tier2activities (tier2_id));
diesel::joinable!(tier2_tier1_kph -> keypoint_hits (kph_id));
diesel::joinable!(tier2_tier1_kph -> tier1activities (tier1_id));
diesel::joinable!(tier2_tier1_kph -> tier2activities (tier2_id));

diesel::allow_tables_to_appear_in_same_query!(
    keypoint_hits,
    keypoints,
    objects,
    poses,
    room_object,
    rooms,
    tier1_activity_objects,
    tier1_activity_poses,
    tier1_activity_rooms,
    tier1activities,
    tier2_activity_poses,
    tier2_tier1_kph,
    tier2activities,
);
