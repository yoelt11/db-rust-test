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

CREATE Table tier1activities (
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
    room_id INTEGER NOT NULL,
    object_id INTEGER NOT NULL,
    PRIMARY KEY (room_id, object_id),
    FOREIGN KEY (room_id) REFERENCES rooms(room_id),
    FOREIGN KEY (object_id) REFERENCES objects(object_id)
);

CREATE Table  tier1_activity_rooms (
    room_id INTEGER NOT NULL,
    tier1_id INTEGER NOT NULL,
    PRIMARY KEY (room_id, tier1_id),
    FOREIGN KEY (room_id) REFERENCES rooms(room_id),
    FOREIGN KEY (tier1_id) REFERENCES tier1activities(tier1_id)
);

CREATE Table  tier1_activity_objects (
    object_id INTEGER NOT NULL,
    tier1_id INTEGER NOT NULL,
    PRIMARY KEY (object_id, tier1_id),
    FOREIGN KEY (object_id) REFERENCES objects(object_id),
    FOREIGN KEY (tier1_id) REFERENCES tier1activities(tier1_id)
);

CREATE Table  tier1_activity_poses (
    pose_id INTEGER NOT NULL,
    tier1_id INTEGER NOT NULL,
    PRIMARY KEY (pose_id, tier1_id),
    FOREIGN KEY (pose_id) REFERENCES poses(pose_id),
    FOREIGN KEY (tier1_id) REFERENCES tier1activities(tier1_id)
);

CREATE Table keypoint_hits(
    kph_id INTEGER PRIMARY KEY NOT NULL,
    object_id INTEGER NOT NULL,
    keypoint_id INTEGER NOT NULL,
    FOREIGN KEY (object_id) REFERENCES objects(object_id),
    FOREIGN KEY (keypoint_id) REFERENCES keypoints(keypoint_id)
);

CREATE Table  tier2_activity_poses (
    tier2_id INTEGER NOT NULL,
    pose_id INTEGER NOT NULL,
    PRIMARY KEY (tier2_id, pose_id),
    FOREIGN KEY (tier2_id) REFERENCES tier2activities(tier2_id)
    FOREIGN KEY (pose_id) REFERENCES poses(pose_id),
);


CREATE Table tier2_tier1_kph (
    tier2_id INTEGER NOT NULL,
    tier1_id INTEGER NOT NULL,
    kph_id INTEGER NOT NULL,
    PRIMARY KEY (tier2_id, tier1_id, kph_id),
    FOREIGN KEY (tier2_id) REFERENCES tier2activities(tier2_id),
    FOREIGN KEY (tier1_id) REFERENCES tier1activities(tier1_id),
    FOREIGN KEY (kph_id) REFERENCES keypoint_hits(kph_id)   
);
