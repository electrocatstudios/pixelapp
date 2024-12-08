function SkeletonManager() {
    this.draw = SkeletonManagerDraw;
    this.update = SkeletonManagerUpdate;
    this.add_animation_limb = SkeletonManagerAddAnimationLimb;
    this.refresh = SkeletonManagerRefresh;
    this.limb_current = [];
    this.limb_list = [];

    this.selected = null;
}

function SkeletonManagerAddAnimationLimb(x,y,rot,length,color, name) {
    var first_pos = new AnimationLimbPosition(x, y , rot, length, 0.0);
    this.limb_list.push(
        new AnimationLimb(name, color, first_pos)
    );
    this.refresh()
}

function SkeletonManagerRefresh() {
    // Refresh the views - limb lists and positions etc

    // First the drop down list
    var output = "<select id='limb_list' onchange='new_limb_selected()'>"
    output += "<option value='none'>--Select A Limb--</option>";
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

    // Second get the moves list
    // TODO: Get the moves list and show it
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
        if(this.limb_list[i].moves_list.length < 2) {
            var nxt = new AnimationLimb("draw_item_" + i, this.limb_list[i].color, this.limb_list[i].get_first());
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
                    // x, y, rot, length, perc
                    var nxt_limb = new AnimationLimbPosition(
                        prev_limb.x + (perc * (next_limb.x - prev_limb.x)),
                        prev_limb.y + (perc * (next_limb.y - prev_limb.y)),
                        prev_limb.rot + (perc * (next_limb.rot - prev_limb.rot)),
                        prev_limb.length + (perc * (next_limb.length - prev_limb.length)),
                        perc
                    )

                    var nxt = new AnimationLimb("draw_item_" + i, this.limb_list[i].color, nxt_limb);
                    this.limb_current.push(nxt);
                }
            }
        }
    }
}