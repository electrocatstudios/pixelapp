var FPS = 30; // target frames per second
var SECONDSBETWEENFRAMES = 1 / FPS;
var ctx = null; // Useful to have a global reference for measuring fonts for instance
var canvas = null; // The main drawing area
var canvas_out = null;
var ctx_out = null;

var currentTime = 0; // Track where we are in the animation loop
var animationLength = null;
var GAME_SIZE = {} ;
var GAME_SIZE_MIN = {x: 300, y: 500}
var OUT_SIZE = {};

var SKELETON_MANAGER = null;
var GUIDEVIEW_MANAGER = null;

function performResize(){
    GAME_SIZE = {
        x: window.innerWidth-20,
        y: window.innerHeight - 90,
        mobile: false
    }

    if(window.picture_width != undefined && window.picture_width < window.innerWidth-20 ){
        GAME_SIZE.x = window.picture_width;   
    } else if (window.picture_width != undefined && window.picture_width >= window.innerWidth-20){
        GAME_SIZE.x = window.innerWidth-20;
        GAME_SIZE.mobile = true;
    }

    if(window.picture_height != undefined){
        GAME_SIZE.y = window.picture_height;
        // Should be able to resize canvas to fit components without having to scroll
    }
    if(GAME_SIZE.x < GAME_SIZE_MIN.x){
      GAME_SIZE.x = GAME_SIZE_MIN.x;
    }
    if(GAME_SIZE.y < GAME_SIZE_MIN.y){
      GAME_SIZE.y = GAME_SIZE_MIN.y;
    }

    $('#canvas').width(GAME_SIZE.x);
    $('#canvas').height(GAME_SIZE.y);
    
    if(canvas != null){
        canvas.width = GAME_SIZE.x;
        canvas.height = GAME_SIZE.y;
    }

    SKELETON_MANAGER = new SkeletonManager();
    animationLength = window.animation_length;

    SKELETON_MANAGER.add_animation_limb(100, 100, 0.0, 50, "#ffff00", "first_test_limb", null);
    
    GUIDEVIEW_MANAGER = new GuideViewManager();
}


$(document).ready(function(){

    if(window.animation_name != undefined && window.animation_name != null && window.animation_name!=""){
      $('#animation_name').html(window.animation_name);
    }

    $('#limb_add_box').addClass('limb_add_box_closed');

    // Do set up
    canvas = document.getElementById('canvas');
    ctx = canvas.getContext('2d');

    performResize();
    
    // The following line sets up the game loop
    setInterval(update, SECONDSBETWEENFRAMES * 500);

});

var preview_frame = 0;
var PREVIEW_COOLDOWN_MAX = 5;
var preview_cooldown = PREVIEW_COOLDOWN_MAX;

function update(){
    if (animationLength === null) {
        return;
    }

    if(playing_animation){
        currentTime += SECONDSBETWEENFRAMES;
        if (currentTime >= (animationLength / 1000)) {
            currentTime -= (animationLength / 1000);
        }    
    }

    // Update interface to show current frame details
    var perc = currentTime / (animationLength / 1000); // Perc range 0-1
    var frame_num = Math.floor(perc * 100);
    $('#animation_position').val(frame_num);
    var frame_idx = Math.floor((GUIDEVIEW_MANAGER.frames.length) * perc);
    var percentage = Math.floor((perc * 100));
    if( frame_idx !== undefined && !(frame_idx>=GUIDEVIEW_MANAGER.frames.length) ) {
        var frame_message = "Frame: " + GUIDEVIEW_MANAGER.frames[frame_idx].frame + ", Percentage: " + percentage + "%";
        $('#cur_frame').html(frame_message);    
    }
    // End interface current frame details

    GUIDEVIEW_MANAGER.update(perc);
    SKELETON_MANAGER.update(perc);

    // Clear the drawing area
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.save();

    ctx.fillRect(0,0,GAME_SIZE.x,GAME_SIZE.y);

    GUIDEVIEW_MANAGER.draw(ctx);
    SKELETON_MANAGER.draw(ctx);

}

function save_animation(){
    var limbs = SKELETON_MANAGER.getAnimationData();
    var guid = window.animation_id;
    var data = {
        guid: guid,
        limbs: limbs,
    }
    var url = "/api/animation_save";
    $.ajax({
        url: url,
        type: 'POST',
        dataType: 'json',
        contentType: "application/json; charset=utf-8",
        data: JSON.stringify(data),
        // beforeSend: function (xhr) {
        //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
        // },
        success: function(ret){
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            // TODO: Post a toast saying we succeeded
            close_menu();
        },
        error: function(ret){
            console.log("ERROR saving pixel data");
            console.log(ret);
        }
    })
}

var playing_animation = true;
function toggle_animation(){
    playing_animation = !playing_animation;
    if(playing_animation){
        currentTime = 0; // Reset the animation on toggle
    } else {
        $('#animation_position').val(Math.floor((currentTime / (animationLength / 1000)) * 100));
    }
}

function update_slider_pos() {
    var perc = $('#animation_position').val() / 100;
    currentTime = (animationLength / 1000) * perc;
}