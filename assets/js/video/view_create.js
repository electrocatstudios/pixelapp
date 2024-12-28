$(document).ready(function() {
    refresh_frames();
});

var frames = [];
var frame_views = [{
    width: 100,
    height: 200,
    x: 100,
    y: 100
}];

function refresh_frames() {
    // Get the frames from the server based on settings
    var guid =window.video_guid;
    var count = $('#frame_count').val();
    
    // Prep query vars
    var start = $('#frame_start').val();
    var end = $('#frame_end').val();
    var vars = "";
    if(start !== undefined && start !== null && start !== "") {
        try{
            var val = parseInt(start);  
            vars = "start="  + val;
        } catch {
            // Do nothing but ignore the error
        }        
    }
    if(vars !== ""){
        vars += "&";
    }
    if(end !== undefined && end !== null && end !== "") {
        try{
            var val = parseInt(end);  
            vars += "end="  + val;
        } catch {
            // Do nothing but ignore the error
        }        
    }
    var url = "/api/frames/" + guid + "/" + count;
    if(vars !== "") {
        url += "?" + vars
    }
    console.log(url);
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret){
            if(ret.status != 'ok'){
                $('#errorfileupload').html(ret.message);
                return;
            }
            // console.log(ret);
            var output = "";
            frames = []; // Reset the values
            
            for(var i=0;i<ret.frames.length;i++){
                if(frame_views[i] === undefined) {
                    frame_views[i] = null;
                }
                var fr = ret.frames[i];
                // console.log(fr)
                var img_url = "/img/videoframe/" + guid + "/" + fr;
                output += "<img width='100px' src='" + img_url + "' style='cursor:pointer;' onclick='load_image(" + i + ")'>";

                var img = new Image();
                img.src = img_url;
                var nxt = {
                    frame: fr,
                    url: img_url,
                    img: img
                }
                frames.push(nxt);
            }
            // console.log(output);
            $('#gallery_view').html(output);
            load_image(0);
            // window.location.href='/';
        },
        error: function(ret){
            console.log("ERROR while getting frame list for video");
            console.log(ret);
        }
    })
}

var cur_sel_frame = null;
function load_image(frame_idx) {
    // Select and reveal the clicked on image on the canvas
    cur_sel_frame = frame_idx;
    var frame = frames[frame_idx];
    // Show the frame_idx in the view finder
    var output = "<div>Image URL: " + frame.url + "</div>";
    output += "<div>Frame Number: " + frame.frame + "</div>";
    $('#image_info').html(output);
    update_view_frame_details();
}

function update_view_frame_details() {
    // Move backwards until we 
    var res = get_actual_frame_view();
    var fv = res[0];
    var idx = res[1];

    $('#box_width').val(fv.width);
    $('#box_height').val(fv.height);
    $('#box_x').val(fv.x);
    $('#box_y').val(fv.y);

    // TODO: show the frame that the image is from
    // for now this isn't configurable - will only take one
    var frame = frames[idx];
    // $('#from_frame').val(frame.frame);
}

function update_view_frame() {
    let cur_frame = cur_sel_frame;
    
    frame_views[cur_frame] = {
        width: parseInt($('#box_width').val()),
        height: parseInt($('#box_height').val()),
        x: parseInt($('#box_x').val()),
        y: parseInt($('#box_y').val())
    }
    
}

function get_actual_frame_view(){
    var fv = frame_views[cur_sel_frame];
    var i = cur_sel_frame;
    for(;fv===null && i>=0;i--) {
        fv = frame_views[i];
        if(fv !== null){
            break;
        }
    }
    return [fv, i];
}
var cur_scale = 1;
function draw_frame(ctx) {
    if (cur_sel_frame == null || frames.length == 0) {
        return;
    }
    var cur_frame = frames[cur_sel_frame];
    var out_width = Math.min(cur_frame.img.width, GAME_SIZE.x);
    var out_height = Math.min(cur_frame.img.height, GAME_SIZE.y);

    var perc_w = out_width / cur_frame.img.width;
    var perc_h = out_height / cur_frame.img.height;
    cur_scale = Math.min(perc_w, perc_h);

    // drawImage(image, sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight)
    ctx.drawImage(
        cur_frame.img,
        0,
        0,
        cur_frame.img.width * cur_scale,
        cur_frame.img.height * cur_scale
        );

    var old_col = ctx.strokeStyle;
    ctx.strokeStyle = "#ffaa44";
    var fv = get_actual_frame_view()[0];
    ctx.beginPath();
    ctx.rect(fv.x * cur_scale, fv.y * cur_scale, fv.width * cur_scale, fv.height * cur_scale);
    ctx.stroke();

    ctx.strokeStyle = "#000000";
    ctx.beginPath();
    ctx.rect((fv.x * cur_scale) - 1, (fv.y * cur_scale) - 1, (fv.width * cur_scale) + 2, (fv.height * cur_scale) + 2);
    ctx.stroke();

    ctx.strokeStyle = old_col;
}

function view_create_mouse_down(x,y){
    var x_act = x / cur_scale;
    var y_act = y / cur_scale;
    // Convert to e
    if(draw_frame_show) {
        frame_views[0].x = parseInt(x_act);
        frame_views[0].y = parseInt(y_act);
        draw_started = true;
    }
    // Now show the new values in the boxes
    update_view_frame_details();
}

function view_create_mouse_up(x,y){
    // We are done, stop the drawing
    if(draw_frame_show){
        draw_frame_show = false;
        $('#canvas').removeClass('draw_cursor');
        $('#canvas').addClass('normal_cursor');        
    }
}

function view_create_mouse_move(x,y){
    var x_act = x / cur_scale;
    var y_act = y / cur_scale;
    // Convert to e
    if(draw_frame_show && draw_started) {
        frame_views[0].width = parseInt(x_act - frame_views[0].x);
        frame_views[0].height = parseInt(y_act - frame_views[0].y);
        // Now show the new values in the boxes
        update_view_frame_details();
    }
}

var draw_frame_show = false;
var draw_started = false;
function draw_frame_toggle() {
    draw_frame_show = !draw_frame_show;

    if(draw_frame_show) {
        $('#canvas').removeClass('normal_cursor');
        $('#canvas').addClass('draw_cursor');
        draw_started = false;
    } else {
        $('#canvas').removeClass('draw_cursor');
        $('#canvas').addClass('normal_cursor');
    }
}