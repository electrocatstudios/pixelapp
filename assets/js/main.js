var FPS = 5; // target frames per second
var SECONDSBETWEENFRAMES = 1 / FPS;
var ctx = null; // Useful to have a global reference for measuring fonts for instance
var canvas = null; // The main drawing area
var canvas_out = null;
var ctx_out = null;

var currentTime = 0; // For debugging - you can store the current time and see how it's changed
var GAME_SIZE = {} ;
var GAME_SIZE_MIN = {x: 300, y: 500}
var OUT_SIZE = {};

var SCALE = 4;
var BOX_WIDTH = 20;

var settings = {"show_background": false}

let PIXEL_MANAGER = null;
let TOOL_MANAGER = null;
let DRAW_MANAGER = null;

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
    // console.log(GAME_SIZE);
    // console.log(GAME_SIZE_MIN);
    

    $('#canvas').width(GAME_SIZE.x);
    $('#canvas').height(GAME_SIZE.y);
    var screen_width = $(document).width();
    var offset_left = (screen_width/2) - (GAME_SIZE.x/2);
    $('#canvas').css({'margin-left':  offset_left + "px"})
   
    if(canvas != null){
        canvas.width = GAME_SIZE.x;
        canvas.height = GAME_SIZE.y;
    }
}

var token = null;
var saved_image_data = null;
var cur_frame = 0;

$(document).ready(function(){
    // $('#color_picker').val("#1234ff");
    // token = getToken()
    // if(token == null){
    //     window.location.href='/login?ret=pixelapp%2f' + window.pixel_id;
    //     return;
    // }

    if(window.pixel_name != undefined && window.pixel_name != null && window.pixel_name!=""){
      $('#pixel_name').html(window.pixel_name);
    }

    if(window.pixel_size!=undefined){
        BOX_WIDTH = window.pixel_size;
    }
    // Do set up
    canvas = document.getElementById('canvas');
    ctx = canvas.getContext('2d');

    canvas_out = document.getElementById('canvas_out');
    ctx_out = canvas_out.getContext('2d');

    DRAW_MANAGER = new DrawManager(ctx, ctx_out);
    TOOL_MANAGER = new ToolManager();

    $('#canvas').click(oncanvasclick);
    $('#canvas').mousemove(onmousemove);
    $('#canvas').on('mouseleave', onmouseleave);
    
    $('#canvas').mousedown(onmousedown);
    $('#canvas').mouseup(onmouseup);

    $('#canvas').on('touchmove', ontouchmove);
    $('#canvas').on('touchstart', ontouchstart);
    $('#canvas').on('touchend', ontouchend);

    $(document).keydown(onKeyDown);
    $(document).keyup(onKeyUp);

    performResize();

    $('#double_pixel_confirm').hide();

    $('#duplicate_pixel_confirm').hide();
    $('#double_pixel_button').show();

    var w = (GAME_SIZE.x/BOX_WIDTH) * SCALE;
    var h = (GAME_SIZE.y/BOX_WIDTH) * SCALE;

    var pix_width = parseInt(window.picture_width/window.pixel_size);
    var pix_height = parseInt(window.picture_height/window.pixel_size);
    saved_image_data = [];
    saved_image_data[0] = new Array(pix_height).fill(null);;
    for(i=0;i<pix_height;i++){
      saved_image_data[0][i] = new Array(pix_width).fill(null);;
    }
    
    PIXEL_MANAGER = new PixelManager();
    
    $('#background_show').on('change', function(){
      settings.show_background = $('#background_show').is(":checked");
    })

    // The following line sets up the game loop
    setInterval(update, SECONDSBETWEENFRAMES * 500);

    // Set up default color
    set_new_color("88","88","88");
});

var preview_frame = 0;
var PREVIEW_COOLDOWN_MAX = 5;
var preview_cooldown = PREVIEW_COOLDOWN_MAX;

function update(){
  // Do update stuff

  // Store the time - for debugging purposes mostly
  currentTime += SECONDSBETWEENFRAMES;

  // Clear the drawing area
  DRAW_MANAGER.update();
  TOOL_MANAGER.update();
  DRAW_MANAGER.draw();

  // draw_image_data();

  // var frame = preview_frame;
  // if(animatepreview){
  //   if(preview_cooldown > 0){
  //     preview_cooldown -= 1;
      
  //   }else{
  //     preview_frame += 1;
  //     preview_cooldown = PREVIEW_COOLDOWN_MAX;

  //     if(preview_frame >= saved_image_data.length){
  //       preview_frame = 0;        
  //     }
      
  //     refresh_preview(preview_frame);
  //   }

  // }else{
  //   refresh_preview(cur_frame);    
  // }
  
}

function clear_all_pixels(){
  // for(var i = 0;i<image_data.data.length;i++){
  //   image_data.data[i] = 0;
  // }
  // for(var i = 0;i<image_data_grid.data.length;i++){
  //   image_data_grid.data[i] = 0;
  // }
}

var bEraserSelected = false;
var bPaintSelected = false;
var bSampleSelected = false;

function select_buttons_clear(){
  $('#paint_button').removeClass('selected_button_black');
  $('#eraser_button').removeClass('selected_button_black');
  $('#sample_button').removeClass('selected_button_black');
  bEraserSelected = false;
  bPaintSelected = false;
  bSampleSelected = false;
}

function select_eraser(){
  select_buttons_clear();
  bEraserSelected = true;
  $('#eraser_button').addClass('selected_button_black');
}

function select_paint(){ 
  select_buttons_clear();
  bPaintSelected = true;
  $('#paint_button').addClass('selected_button_black');
}
function select_sample(){
  select_buttons_clear();
  bSampleSelected = true;
  $('#sample_button').addClass('selected_button_black');
}

function save_image(){
    window.open(canvas_out.toDataURL('image/png'));
}

function save_pixels(){
    var pixels = PIXEL_MANAGER.getPixelData();
    var shaders = PIXEL_MANAGER.getShaderData();
    var guid = window.pixel_id;
    var data = {
        guid: guid,
        pixels: pixels,
        shaders: shaders
    }
    var url = "/api/save";
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

function scale(val){
  // console.log(val);
  SCALE = val;
  $('#canvas-out').width(40*val);
  $('#canvas-out').height(30*val);
  $('#canvas_out').width(40*val);
  $('#canvas_out').height(30*val);

  clear_all_pixels();

  OUT_SIZE = new Location(40*val,30*val);
  canvas_out = document.getElementById('canvas_out');
  ctx_out = canvas_out.getContext('2d');
  // console.log(OUT_SIZE);
  // image_data = ctx_out.createImageData(OUT_SIZE.x,OUT_SIZE.y);
  load_frame(cur_frame+1);
}

function load_frame(frame_number){
  DRAW_MANAGER.loadFrame(frame_number);
  TOOL_MANAGER.refreshFrameButtons();
}

function new_frame(){

  PIXEL_MANAGER.newFrame();
  
  // Make sure buttons reflect actual frames
  TOOL_MANAGER.refreshFrameButtons();
}

function delete_frame() {
  PIXEL_MANAGER.deleteFrame();

  // Make sure buttons reflect actual frames
  TOOL_MANAGER.refreshFrameButtons();
}

function toggle_animation(){
  DRAW_MANAGER.animatepreview = !DRAW_MANAGER.animatepreview;
  if(DRAW_MANAGER.animatepreview){
    $('#animation_button').addClass('selected_button_black')
  }else{
    $('#animation_button').removeClass('selected_button_black')
  }
}

// Double pixel density
function show_confirm_pixel(){
  $('#double_pixel_confirm').show();
  $('#duplicate_pixel_confirm').hide();
}

function confirmed_pixel_double(){
  var data = {
    multiplyingfactor: 2
  };

  var url = "/api/double/" + window.pixel_id;
  $.ajax({
    url: url,
    data: JSON.stringify(data),
    type: 'POST',
    dataType: 'json',
    contentType: "application/json; charset=utf-8",
    // beforeSend: function (xhr) {
    //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
    // },
    success: function(ret){
      // console.log(ret);
      if(ret.status != "ok"){
          console.log(ret.message);
          $('#error').html(ret.message);
          return;
      }

      window.location.href='/pixel/' + window.pixel_id;
      
    },
    error: function(ret){
        console.log("ERROR while getting saved data");
        console.log(ret);
    }
  })
}

function duplicate_image() {
  var newname = $('#duplicate_pixel_name').val();
  var data = {
    newimagename: newname,
  };
  console.log(data);
  var url = "/api/duplicate/" + window.pixel_id;
  $.ajax({
    url: url,
    data: JSON.stringify(data),
    type: 'POST',
    dataType: 'json',
    contentType: "application/json; charset=utf-8",
    success: function(ret) {
      if(ret.status != "ok"){
        console.log(ret.message);
        $('#error').html(ret.message);
        return;
      }
      window.location.href='/pixel/' + window.pixel_id;
    },
    error: function(err) {
      console.log("Error while duplicating image");
    }
  });

}

function show_duplicate_image() {  
  console.log("Showing the duplication screen");
  $('#double_pixel_confirm').hide();
  $('#duplicate_pixel_confirm').show();
}

function cancel_pixel_double(){
  $('#double_pixel_confirm').hide();
  $('#duplicate_pixel_confirm').hide();
  
  $('#double_pixel_button').show();
}