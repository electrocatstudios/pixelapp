function PreviewViewManager() {
    this.refresh = PreviewViewManagerRefresh;

    this.update = PreviewViewManagerUpdate;
    this.draw = PreviewViewManagerDraw;

    this.add_frame = PreviewViewManagerAddFrame;

    this.frames = [];

    this.refresh();
}

function PreviewViewManagerRefresh() {
    if(window.view_guid === undefined || window.view_guid === null || window.view_guid === ""){
        // Do nothing there is no view attached
        console.log("No view guid on preview page")
        return;
    }

    this.frames = [];

     // Load images from view
     var url = "/api/view/" + window.view_guid;
     $.ajax({
         url: url,
         type: 'GET',
         dataType: 'json',
         success: function(ret){
             if(ret.status != 'ok'){
                 $('#error').html(ret.message);
                 return;
             }
             for(var i=0;i<ret.frames.length;i++){
                PREVIEWVIEW_MANAGER.add_frame(ret.frames[i]);
             }
         },
         error: function(ret){
             console.log("ERROR creating new collection");
             console.log(ret);
         }
     })
}

function PreviewViewManagerUpdate(perc) {
    // Do updates - calculate current frame showing
    this.cur_sel_frame = Math.floor((this.frames.length - 1)* perc)
}

function PreviewViewManagerDraw(ctx) {
    if(this.cur_sel_frame === undefined 
        || this.cur_sel_frame === null 
        || this.cur_sel_frame > this.frames.length 
        || this.frames[this.cur_sel_frame] === undefined) 
    {
        return;
    }
    var cur_frame = this.frames[this.cur_sel_frame].image;
    if(cur_frame === undefined || cur_frame.width === undefined){
        return; // Not ready yet.
    }
    // TODO: Stretch or compress image as required
    var width_diff = window.picture_width / cur_frame.width;
    if(width_diff > 1){
        width_diff = 1;
    }
    var height_diff = window.picture_height / cur_frame.height;
    if(height_diff > 1){
        height_diff = 1;
    }
    var scale = 1;
    if (height_diff>width_diff) {
        // Width is more compressed
        scale = width_diff;
    } else {
        scale = height_diff;
    }
    var output_width = cur_frame.width * scale;
    var output_height = cur_frame.height * scale;
    
    var offset_x = (GAME_SIZE.x / 2) - (output_width / 2);
    var offset_y = (GAME_SIZE.y / 2) - (output_height / 2);
    ctx.drawImage(
        cur_frame,
        0,
        0,
        cur_frame.width,
        cur_frame.height,
        offset_x,
        offset_y,
        output_width,
        output_height
    );
}

function PreviewViewManagerAddFrame(frame){
    var nxt = {
        image: new Image(),
        frame: frame
    }
    this.frames.push(nxt);
    this.frames[this.frames.length-1].image.src = "/img/viewframe/" + window.view_guid + "/" + frame;
}