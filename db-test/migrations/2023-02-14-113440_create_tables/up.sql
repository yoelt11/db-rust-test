-- Your SQL goes here
-- Main Tables
CREATE Table rooms (
    room_id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    UNIQUE (name)
);

CREATE Table objects (
    object_id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    UNIQUE (name)
);

CREATE Table keypoints (
    keypoint_id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    UNIQUE (name)
);

CREATE Table poses (
    pose_id INTEGER PRIMARY KEY NOT NULL,
    name Text NOT NULL,
    UNIQUE (name)
 );

CREATE Table tier1_idactivities (
    tier1_id INTEGER PRIMARY KEY NOT NULL,
    tier1 Text NOT NULL,
    UNIQUE (tier1)
);

CREATE Table tier2activities (
    tier2_id INTEGER PRIMARY KEY NOT NULL,
    tier2 Text NOT NULL,
    UNIQUE (tier2)
);

-- Relationship Tables
CREATE Table  room_object (
    room_id INTEGER,
    object_id INTEGER,
    PRIMARY KEY (room_id, object_id),
    FOREIGN KEY (room_id) REFERENCES rooms(room_id),
    FOREIGN KEY (object_id) REFERENCES objects(object_id)
);

CREATE Table  tier1_idactivity_rooms (
    room_id INTEGER,
    tier1_id INTEGER,
    PRIMARY KEY (room_id, tier1_id),
    FOREIGN KEY (room_id) REFERENCES rooms(room_id),
    FOREIGN KEY (tier1_id) REFERENCES tier1activities(tier1_id)
);

CREATE Table  tier1_idactivity_poses (
    pose_id INTEGER,
    tier1_id INTEGER,
    PRIMARY KEY (pose_id, tier1_id),
    FOREIGN KEY (pose_id) REFERENCES poses(pose_id),
    FOREIGN KEY (tier1_id) REFERENCES tier1activities(tier1_id)
);

CREATE Table  tier1activity_tier2activity (
    tier1_id INTEGER,
    tier2_id INTEGER,
    PRIMARY KEY (tier1_id, tier2_id),
    FOREIGN KEY (tier1_id) REFERENCES tier1activities(tier1_id),
    FOREIGN KEY (tier2_id) REFERENCES tier2activities(tier2_id)
);

CREATE Table tier2_objects_keypoints (
    tier2_id INTEGER,
    object_id INTEGER,
    keypoint_id INTEGER,
    PRIMARY KEY (tier2_id, object_id, keypoint_id),
    FOREIGN KEY (tier2_id) REFERENCES tier2activities(tier2_id),
    FOREIGN KEY (object_id) REFERENCES objects(object_id),
    FOREIGN KEY (keypoint_id) REFERENCES keypoints(keypoint_id)   
);