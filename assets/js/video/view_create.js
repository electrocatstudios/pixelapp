$(document).ready(function() {
    refresh_frames();
});

let frames = [];

function refresh_frames() {
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
    let url = "/api/frames/" + guid + "/" + count;
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
                let fr = ret.frames[i];
                // console.log(fr)
                let img_url = "/img/videoframe/" + guid + "/" + fr;
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
    cur_sel_frame = frame_idx;
    // Show the frame_idx in the view finder
}

function draw_frame(ctx) {
    if (cur_sel_frame == null || frames.length == 0) {
        return;
    }
    var cur_frame = frames[cur_sel_frame];
    var out_width = Math.min(cur_frame.img.width, GAME_SIZE.x);
    var out_height = Math.min(cur_frame.img.height, GAME_SIZE.y);

    var perc_w = out_width / cur_frame.img.width;
    var perc_h = out_height / cur_frame.img.height;
    var perc = Math.min(perc_w, perc_h);
    // drawImage(image, sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight)
    ctx.drawImage(
        cur_frame.img,
        0,
        0,
        cur_frame.img.width * perc,
        cur_frame.img.height * perc
        );

    
}