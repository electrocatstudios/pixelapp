let MAX_ANIMATION_COOLDOWN= 3;
    
function DrawManager(ctx, ctx_out){
    this.ctx = ctx;
    this.ctx_out = ctx_out;
    this.view_list = {
        grid: true,
        guidelines: true,
        background: true,
        foreground: true,
        shader: false,
        animation: true
    }
    this.update = DrawManagerUpdate;
    this.draw = DrawManagerDraw;
    this.updatePreview = DrawingManagerUpdatePreview;
    this.loadFrame = DrawManagerLoadFrame;

    this.cur_frame = 0;
    this.background_color = "#000000";
    this.alpha_channel = 1.0;
    this.alpha_channel_shader = 0.5;

    this.performResize = DrawManagerPerformResize;

    this.position = {x:0,y:0};
    this.move = DrawingManagerMove;
    this.resetOrigin = DrawManagerResetOrigin;
    this.toggleView = DrawManagerToggleView;

    this.animatepreview = false;
    this.animation_frame = 0;
    this.animation_cooldown = MAX_ANIMATION_COOLDOWN;

    this.animation_guide = [];
}

function DrawManagerUpdate(){
    if(window.animation_id !== undefined && window.animation_id !== null && window.animation_id !== ""){
        let num_frames = PIXEL_MANAGER.saved_image_data.length
        if(this.animation_guide.length < num_frames) {
            this.animation_guide = []; // Refresh the whole list - size has changed
            for(var i=0;i<num_frames;i++){
                var nxt = new Image();
                var src = '/img/animation_render/' + window.animation_id + "/" + i + "/" + (num_frames - 1);
                nxt.src = src;
                this.animation_guide.push(nxt);
            }
        }
    }
}

// Pass in null to ignore that one
// otherwise move the x and y by the amount given
function DrawingManagerMove(x, y){
    let dims = PIXEL_MANAGER.getDimensions()
    let max_x = dims.box_width * (dims.max_x-1);
    let max_y = dims.box_width * (dims.max_y-1);

    if(x != null){
        this.position.x += x;
        if(this.position.x < max_x * -1){
            this.position.x = max_x * -1;
        }
        if(this.position.x > max_x){
            this.position.x = max_x;
        }
    }
    
    if(y != null){
        this.position.y += y;
        if(this.position.y < max_y * -1){
            this.position.y = max_y * -1;
        }
        if(this.position.y > max_y){
            this.position.y = max_y;
        }
    }
}

function DrawManagerDraw(){
    let ctx = this.ctx;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.save();

    var w = GAME_SIZE.x;
    var h = GAME_SIZE.y;
    // Background based on settings
    if(this.view_list.background){
        ctx.fillStyle = DRAW_MANAGER.background_color;
        ctx.fillRect(0,0,w,h);
    }

    // Draw animation guide
    if(window.animation_guid !== "" 
        && this.view_list.animation 
        && this.animation_guide.length ==  PIXEL_MANAGER.saved_image_data.length ) {
        ctx.drawImage(
            this.animation_guide[this.cur_frame],
            0,
            0,
            this.animation_guide[this.cur_frame].width,
            this.animation_guide[this.cur_frame].height
            );
    }

    //vertical bars
    let dims = PIXEL_MANAGER.getDimensions();
    if(dims == undefined){
        console.log("Error: undefined dimensions");
        return;
    }

    // Draw the grid
    if(this.view_list.grid){
        ctx.strokeStyle = "#f0f0f0"
        ctx.beginPath();

        for(var i = dims.box_width; i <= (dims.max_x+1)*dims.box_width; i+=dims.box_width){
            ctx.moveTo(i + dims.offset_x + this.position.x,0);
            ctx.lineTo(i + dims.offset_x+this.position.x,GAME_SIZE.y);
        }
    
        //horizontal bars
        for(var i = dims.box_width; i <= (dims.max_y+1)*dims.box_width; i+=dims.box_width){
            ctx.moveTo(0,i + dims.offset_y + this.position.y);
            ctx.lineTo(GAME_SIZE.x,i+ dims.offset_y + this.position.y);
        }
        ctx.stroke();
    }

    if(this.view_list.foreground){
        let image_data = PIXEL_MANAGER.getFrame(this.cur_frame);
    
        for(var y=0;y<image_data.length;y++){
            for(var x=0;x<image_data[y].length;x++){
                var pix = image_data[y][x];
                if(pix != null){
                    var clr = 'rgba(' + pix.r +
                    ',' + pix.g +
                    ',' + pix.b +
                    ',' + pix.alpha + ')';
                    ctx.fillStyle = clr;
            
                    let x_loc = x*dims.box_width;
                    let y_loc = y*dims.box_width;
                    
                    ctx.globalAlpha = pix.alpha;
                    ctx.fillRect(x_loc + dims.image_offset_x + this.position.x,y_loc + dims.image_offset_y + this.position.y,dims.box_width,dims.box_width);
                    ctx.globalAlpha = 1;
                }
            }
        }
    }
    if(this.view_list.shader){
        let image_data = PIXEL_MANAGER.getFrameShader(this.cur_frame);
    
        for(var y=0;y<image_data.length;y++){
            for(var x=0;x<image_data[y].length;x++){
                var pix = image_data[y][x];
                if(pix != null){
                    var clr = 'rgba(' + pix.r +
                    ',' + pix.g +
                    ',' + pix.b +
                    ',' + pix.alpha + ')';
                    ctx.fillStyle = clr;
            
                    let x_loc = x*dims.box_width;
                    let y_loc = y*dims.box_width;
                    
                    ctx.globalAlpha = pix.alpha;
                    ctx.fillRect(x_loc + dims.image_offset_x + this.position.x,y_loc + dims.image_offset_y + this.position.y,dims.box_width,dims.box_width);
                    ctx.globalAlpha = 1;
                }
            }
        }
    }
    
    this.updatePreview();
}

function DrawingManagerUpdatePreview(){
    let ctx_out = this.ctx_out;

    ctx_out.clearRect(0, 0, canvas_out.width, canvas_out.height);
    ctx_out.save();
    var oldFill = ctx_out.fillStyle;

    // Get current frame
    let image_data = null;
    if(this.animatepreview){
        // Do animation updates
        if(this.animation_cooldown < 1){
            this.animation_cooldown = MAX_ANIMATION_COOLDOWN;
            this.animation_frame += 1;
            if(this.animation_frame > PIXEL_MANAGER.getNumberFrames() - 1){
                this.animation_frame = 0;
            }
        }else{
            this.animation_cooldown -= 1;
        }
        image_data = PIXEL_MANAGER.getFrame(this.animation_frame);
    }else{
        image_data = PIXEL_MANAGER.getFrame(this.cur_frame);
    }
        
    for(var y=0;y<image_data.length;y++){
        for(var x=0;x<image_data[y].length;x++){
            var p = image_data[y][x];
            if(p == null || p.alpha == 0){
                continue;
            }
            var red = p.r.toString(16);
            if(red.length < 2){
                red = "0" + red;
            }
            var green = p.g.toString(16);
            if(green.length < 2){
                green = "0" + green;
            }
            var blue = p.b.toString(16);
            if(blue.length < 2){
                blue = "0" + blue;
            }
            var col = "#" + red + green + blue;
            ctx_out.fillStyle = col;
            // console.log(col);
            
            ctx_out.fillRect(x*SCALE, y*SCALE,SCALE,SCALE);
        }
    }

    ctx_out.fillStyle = oldFill;
}

function DrawManagerPerformResize(){

}

function DrawManagerLoadFrame(frame_number){
    // TODO: Perform checks to make sure not out of bounds
    this.cur_frame = frame_number-1;
}

function DrawManagerResetOrigin(){
    this.position.x = 0;
    this.position.y = 0;
}

function DrawManagerToggleView(btn){
    this.view_list[btn] = !this.view_list[btn];
    if(this.view_list[btn]){
        $('#toolbar_view_select_' + btn).addClass('toolbar_view_button_selected');
    }else{
        $('#toolbar_view_select_' + btn).removeClass('toolbar_view_button_selected');
    }
}
