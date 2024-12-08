var FPS = 5; // target frames per second
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
        // TODO: Take into account screen height minus components
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
    var screen_width = $(document).width();
    var offset_left = (screen_width/2) - (GAME_SIZE.x/2);
    $('#canvas').css({'margin-left':  offset_left + "px"})
   
    if(canvas != null){
        canvas.width = GAME_SIZE.x;
        canvas.height = GAME_SIZE.y;
    }

    SKELETON_MANAGER = new SkeletonManager();
    animationLength = window.animation_length;
}


$(document).ready(function(){

    if(window.animation_name != undefined && window.animation_name != null && window.animation_name!=""){
      $('#animation_name').html(window.animation_name);
    }

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

    // Store the time - for debugging purposes mostly
    currentTime += SECONDSBETWEENFRAMES;
    if (currentTime >= animationLength) {
        currentTime -= animationLength
    }


    // Clear the drawing area

}