function AnimationLimb(name, col, startPos, parent){
    // startPos is first position
    this.name = name;
    this.color = col;
    this.moves_list = [startPos];
    this.parent = parent;

    this.add_position = AnimationLimbAddPosition;
    this.remove_position = AnimationLimbDelPosition;
    this.get_first = function() { return this.moves_list[0] };
}

function AnimationLimbAddPosition(x, y, rot, length, perc) {
    this.moves_list.push(new AnimationLimbPosition(x, y, rot, length, perc));
}

function AnimationLimbDelPosition(index) {
    this.moves_list = this.moves_list.splice(index, 1);
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