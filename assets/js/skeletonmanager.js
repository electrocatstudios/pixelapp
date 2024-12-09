function SkeletonManager() {
    this.draw = SkeletonManagerDraw;
    this.update = SkeletonManagerUpdate;
    this.add_animation_limb = SkeletonManagerAddAnimationLimb;
    this.refresh = SkeletonManagerRefresh;
    this.limb_current = [];
    this.limb_list = [];

    this.update_limb_move = SkeletonManagerUpdateLimbMove;
    this.delete_limb_move = SkeletonManagerRemoveLimbMove;
    this.selected = null;
}

function SkeletonManagerAddAnimationLimb(x,y,rot,length,color, name, parent) {
    var first_pos = new AnimationLimbPosition(x, y , rot, length, 0.0);
    this.limb_list.push(
        new AnimationLimb(name, color, first_pos, parent)
    );
    this.refresh()
}

function SkeletonManagerRefresh() {
    // Refresh the views - limb lists and positions etc

    // First the drop down list
    
    var output = "<select id='limb_list' onchange='new_limb_selected()'><option value='none'>--Select A Limb--</option>";
    for(var i=0;i<this.limb_list.length;i++){
        var l = this.limb_list[i];
        output += "<option value='" + l.name + "'";
        if(this.selected === l.name) {
            output += "selected";
        }
        output += ">" + l.name + "</option>"
    }
    output += "</select>"
    $('#limb_select').html(output);

    var parent_output = "<select id='parent_selector'><option value='none'>--None--</option>";
    for(var i=0;i<this.limb_list.length;i++){
        var l = this.limb_list[i];
        parent_output += "<option value='" + l.name + "'>" + l.name + "</option>";
    }
    parent_output += "</select>"
    $('#limb_add_parent').html(parent_output);

    // Second get the moves list
    if (self.selected === null){
        return;
    }
    for(var i=0;i<this.limb_list.length;i++){
        if(this.limb_list[i].name === this.selected){
            var limb = this.limb_list[i];
            var output = ""
            for(var j=0;j<limb.moves_list.length;j++){
                var mov = limb.moves_list[j];
                output += get_move_card(mov, j);
            }
            output += get_new_move_card();
            $('#limb_positions').html(output);
            break;
        }
    }
}

function get_move_card(mov, idx){
    var ret = "<div class='move_card'>"
    ret += "X: <input type'number' id='move_x_" + idx + "' value='" + mov.x + "'>";
    ret += "Y: <input type'number' id='move_y_" + idx + "' value='" + mov.y + "'>";
    ret += "Rotation: <input type'number' id='move_rot_" + idx + "' value='" + mov.rot + "'>";
    ret += "Length: <input type'number' id='move_length_" + idx + "' value='" + mov.length + "'>";
    ret += "Perc: <input type'number' id='move_perc_" + idx + "' value='" + mov.perc + "'>";
    ret += "<button onclick='update_limb_move(" + idx + ")'>Update</button>";
    ret += "<button onclick='delete_limb_move(" + idx + ")'>Delete</button>";
    ret += "</div>"
    return ret;
}

function get_new_move_card() {
    var ret = "<div class='move_card'>"
    ret += "X: <input type'number' id='move_x_new' placeholder='X'>";
    ret += "Y: <input type'number' id='move_y_new' placeholder='Y'>";
    ret += "Rotation: <input type'number' id='move_rot_new' placeholder='Rot'>";
    ret += "Length: <input type'number' id='move_length_new' placeholder='Length'>";
    ret += "Perc: <input type'number' id='move_perc_new' placeholder='Perc'>";
    ret += "<button onclick='new_limb_move()'>Add</button>";
    ret += "</div>"
    return ret;
}

function new_limb_selected() {
    var val = $('#limb_list').val();
    if(val === "none") {
        SKELETON_MANAGER.selected = null;
    } else {
        SKELETON_MANAGER.selected = val;
    }
    // console.log(SKELETON_MANAGER.selected);
    SKELETON_MANAGER.refresh();
}

function SkeletonManagerDraw(ctx) {
    for(var i=0; i<this.limb_current.length;i++){
        var limb = this.limb_current[i];
        var pos = limb.get_first();
        ctx.beginPath();
        var old_stroke_style = ctx.strokeStyle;
        
        ctx.strokeStyle = limb.color;
        ctx.moveTo(pos.x, pos.y);
        ctx.lineTo(
            pos.x + (pos.length * Math.sin(pos.rot)),
            pos.y + (pos.length * Math.cos(pos.rot))
        );
        ctx.stroke();
        ctx.strokeStyle = old_stroke_style;
    }
}

function SkeletonManagerUpdate(delta) {
    if(delta < 0.0 || delta > 1.0) {
        console.log("Passed delta", delta, "is out of bounds (0<d<1)")
    }

    // Calculate the current positions
    this.limb_current = [];

    for(var i=0;i<this.limb_list.length;i++){
        // Find parent location
        var nxt_adjust = {x: 0, y: 0};
        if(this.limb_list[i].parent !== null){
            for(var j=0;j<this.limb_current.length;j++){
                if(this.limb_current[j].parent === this.limb_list[i].parent) {
                    var cur_move = this.limb_current[j].get_first();
                    nxt_adjust.x = cur_move.get_end_x();
                    nxt_adjust.y = cur_move.get_end_y();
                    break;
                }
            }
        }

        if(this.limb_list[i].moves_list.length < 2) {
            let next_limb_pos = this.limb_list[i].get_first().copy();
            next_limb_pos.x += nxt_adjust.x;
            next_limb_pos.y += nxt_adjust.y
            var nxt = new AnimationLimb("draw_item_" + i, this.limb_list[i].color, next_limb_pos, this.limb_list[i].name);
            this.limb_current.push(nxt);
        } else {
            // Calculate where in the scale we are. 
            // Find the two limbs between which the given delta is
            for(var j = 1; j<this.limb_list[i].moves_list.length;j++){
                var prev_limb = this.limb_list[i].moves_list[j-1];
                var next_limb = this.limb_list[i].moves_list[j];

                if(prev_limb.perc < delta && next_limb.perc > delta) {
                    // Where we fall in between the two points
                    var perc = (delta - prev_limb.perc) / (next_limb.perc - prev_limb.perc);
                    // Args are: x, y, rot, length, perc
                    var nxt_limb = new AnimationLimbPosition(
                        nxt_adjust.x + prev_limb.x + (perc * (next_limb.x - prev_limb.x)),
                        nxt_adjust.y + prev_limb.y + (perc * (next_limb.y - prev_limb.y)),
                        prev_limb.rot + (perc * (next_limb.rot - prev_limb.rot)),
                        prev_limb.length + (perc * (next_limb.length - prev_limb.length)),
                        perc
                    )

                    var nxt = new AnimationLimb("draw_item_" + i, this.limb_list[i].color, nxt_limb, this.limb_list[i].name);
                    this.limb_current.push(nxt);
                }
            }
        }
    }
}

function submit_new_limb() {
    // Called from HTML to submit a new limb
    $('#new_limb_error').html("");

    var name = $('#new_limb_name').val();
    var x = $('#new_limb_x').val();
    var y = $('#new_limb_y').val();
    var rot = $('#new_limb_rot').val();
    var length = $('#new_limb_length').val();
    var color = $('#new_limb_color').val();

    // Check null vals
    if(name === undefined || name === null || name === "") {
        $('#new_limb_error').html("Name cannot be empty");
        return;
    } else if (name.toLowerCase() === "none") {
        $('#new_limb_error').html("Name cannot be 'none'");
        return;
    }
    if(x === undefined || x === null) {
        $('#new_limb_error').html("X value required");
        return;
    }
    if(y === undefined || y === null) {
        $('#new_limb_error').html("Y value required");
        return;
    }
    if(length === undefined || length === null) {
        $('#new_limb_error').html("Length value required");
        return;
    }
    if(rot === undefined || rot === null) {
        rot = 0;
    }

    // Check values are correct type
    try {
        x = parseFloat(x);
        y = parseFloat(y);
        length = parseFloat(length);
        rot = parseFloat(rot);
    } catch (err) {
        $('#new_limb_error').html("One or more values are not valid numbers");
    }

    var parent = $('#parent_selector').val();
    if(parent === 'none'){
        parent = null;
    }

    SKELETON_MANAGER.add_animation_limb(x,y,rot,length,color,name, parent);

    $('#new_limb_name').val("");
    $('#new_limb_x').val("");
    $('#new_limb_y').val("");
    $('#new_limb_rot').val("");
    $('#new_limb_length').val("");
    $('#new_limb_color').val("");
    close_new_limb();
}

var limb_add_box_open = false;

function add_new_limb() {
    limb_add_box_open = !limb_add_box_open;
    if(limb_add_box_open){
        $('#limb_add_box').removeClass('limb_add_box_closed');
    } else {
        $('#limb_add_box').addClass('limb_add_box_closed');
    }
}

function close_new_limb() {
    $('#limb_add_box').addClass('limb_add_box_closed');
    limb_add_box_open = false;
}

function update_limb_move(idx) {
    SKELETON_MANAGER.update_limb_move(idx);
}

function delete_limb_move(idx) {
    SKELETON_MANAGER.delete_limb_move(idx);
}

function new_limb_move() {
    SKELETON_MANAGER.update_limb_move("new");
}

function SkeletonManagerUpdateLimbMove(idx) {
    if(this.selected === null) {
        console.log("No limb selected");
        return;
    }

    // Get the updated values
    var x = $('#move_x_' + idx).val();
    x = parseFloat(x);
    var y = $('#move_y_' + idx).val();
    y = parseFloat(y);
    var rot = $('#move_rot_' + idx).val();
    rot = parseFloat(rot);
    var length = $('#move_length_' + idx).val();
    length = parseFloat(length);
    var perc = $('#move_perc_' + idx).val();
    perc = parseFloat(perc);
    
    for(var i=0;i<this.limb_list.length;i++) {
        var limb = this.limb_list[i];
        if(limb.name === this.selected) {
            if(idx === "new"){
                limb.add_position(x, y, rot, length, perc);
            } else {
                limb.update_position(idx, x, y, rot, length, perc);
            }
        }
    }
}

function SkeletonManagerRemoveLimbMove(idx) {
    if(this.selected === null) {
        console.log("No limb selected");
        return;
    }
    for(var i=0;i<this.limb_list.length;i++) {
        var limb = this.limb_list[i];
        if(limb.name === this.selected) {
            limb.remove_position(idx);
        }
    }
}