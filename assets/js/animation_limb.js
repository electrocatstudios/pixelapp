function AnimationLimb(name, col, startPos, parent){
    // startPos is first position
    this.name = name;
    this.color = col;
    this.moves_list = [startPos];
    this.parent = parent;

    this.add_position = AnimationLimbAddPosition;
    this.update_position = AnimationLimbUpdatePosition;
    this.remove_position = AnimationLimbDelPosition;
    this.get_first = function() { return this.moves_list[0] };

}

function AnimationLimbAddPosition(x, y, rot, length, perc) {
    this.moves_list.push(new AnimationLimbPosition(x, y, rot, length, perc));
    this.moves_list.sort((a, b) => a.perc - b.perc)
    SKELETON_MANAGER.refresh();
}

function AnimationLimbDelPosition(idx) {
    if(idx < 0 || idx >= this.moves_list.length) {
        console.log("ERROR: We've been passed an index out of range while updating the limb");
        return;
    }
    this.moves_list.splice(idx, 1); // this.moves_list = t
    SKELETON_MANAGER.refresh();
}

function AnimationLimbUpdatePosition(idx, x, y, rot, length, perc) {
    if(idx < 0 || idx >= this.moves_list.length) {
        console.log("ERROR: We've been passed an index out of range while updating the limb");
        return;
    }
    this.moves_list[idx].x = x;
    this.moves_list[idx].y = y;
    this.moves_list[idx].rot = rot;
    this.moves_list[idx].length = length;
    this.moves_list[idx].perc = perc;
    SKELETON_MANAGER.refresh();
}

function AnimationLimbPosition(x, y, rot, length, perc) {
    // (X,Y) start point
    // rot - rotation in radians
    // length - to get point 2
    // perc - up to percentange
    this.x = x;
    this.y = y;
    this.rot = rot;
    this.length = length;
    this.perc = perc;

    this.get_end_x = () => {return this.x + (this.length * Math.sin(this.rot))};
    this.get_end_y = () => {return this.y + (this.length * Math.cos(this.rot))};
    this.copy = () => {return new AnimationLimbPosition(this.x, this.y, this.rot, this.length, this.perc)}
}