var FPS = 30; // target frames per second
var SECONDSBETWEENFRAMES = 1 / FPS;
var ctx = null; // Useful to have a global reference for measuring fonts for instance
var canvas = null; // The main drawing area
var canvas_out = null;
var ctx_out = null;

var currentTime = 0; // Track where we are in the animation loop
var animationLength = 2000;
var GAME_SIZE = {} ;
var GAME_SIZE_MIN = {x: 300, y: 500}
var OUT_SIZE = {};

var PREVIEWVIEW_MANAGER = null;

function performResize(){
    GAME_SIZE = {
        x: window.innerWidth-30,
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

}

$(document).ready(function(){
    // Do set up
    canvas = document.getElementById('canvas');
    ctx = canvas.getContext('2d');

    performResize();

    // The following line sets up the game loop
    setInterval(update, SECONDSBETWEENFRAMES * 500);

    $('#canvas').on('mousedown', onmousedown_view);
    $('#canvas').on('mouseup', onmouseup_view);
    $('#canvas').on('mousemove', onmousemove_view);

    PREVIEWVIEW_MANAGER = new PreviewViewManager();
})

/* Mouse and keyboard functions */
function onmousedown_view(evt){
    var x = evt.pageX - canvas.offsetLeft; // Position on the canvas X
    var y = evt.pageY - canvas.offsetTop;// Position on the canvas Y
    // view_create_mouse_down(x,y);
}

function onmouseup_view(evt) {
    // view_create_mouse_up();
    var x = evt.pageX - canvas.offsetLeft; // Position on the canvas X
    var y = evt.pageY - canvas.offsetTop;// Position on the canvas Y
    // view_create_mouse_up(x,y);   
}

function onmousemove_view(evt) {
    var x = evt.pageX - canvas.offsetLeft; // Position on the canvas X
    var y = evt.pageY - canvas.offsetTop;// Position on the canvas Y
    $('#debug').html("X: " + x + ", Y:" + y);
    // view_create_mouse_move(x,y);
}
/* End mouse and keyboard functions */

function update(){
    // Clear the drawing area
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.save();

    ctx.fillRect(0,0,GAME_SIZE.x,GAME_SIZE.y);
    
    currentTime += SECONDSBETWEENFRAMES;
    if (currentTime >= (animationLength / 1000)) {
        currentTime -= (animationLength / 1000);
    }    
    var perc = currentTime / (animationLength / 1000);
    PREVIEWVIEW_MANAGER.update(perc);

    PREVIEWVIEW_MANAGER.draw(ctx);
}


var frames = [];