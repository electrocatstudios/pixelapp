var aryCurPressedKeys = new Array();

function onKeyDown(evt){
  var key=String.fromCharCode(evt.which||evt.keyCode);
  // If key not already down then add to our list
  if(TOOL_MANAGER.keyDown(key)){
    return;
  }
  if(!isKeyPressed(key)){
    aryCurPressedKeys[aryCurPressedKeys.length]=key;
  }
}

function onKeyUp(evt){
  var key=String.fromCharCode(evt.which||evt.keyCode);
  // If the key released is in our list then remove it
  for(var i=0;i<aryCurPressedKeys.length;i++){
    if(key==aryCurPressedKeys[i]){
      removeArrayItem(aryCurPressedKeys,i);
    }
  }
}

function isKeyPressed(key){
  for(var i=0;i<aryCurPressedKeys.length;i++){
    if(aryCurPressedKeys[i]==key){
      return true;
    }
  }

  return false;
}

//Move all items in the array above the point down and then
//delete the last item.
function removeArrayItem(_array,nItem){
  for(var i=nItem;i<_array.length;i++){
    _array[i]=_array[i+1];

    if(i==_array.length-1){
      delete _array[_array.length];
      return;
    }
  }
}

function removeAllKeysFromArray(){
  aryCurPressedKeys = new Array();
}

function oncanvasclick(evt){
    let dims = PIXEL_MANAGER.getDimensions();

    var x = evt.pageX - canvas.offsetLeft; // Position on the canvas X
    x -= dims.image_offset_x + DRAW_MANAGER.position.x;
    x = Math.floor(x/dims.box_width);

    var y = evt.pageY - canvas.offsetTop;// Position on the canvas Y
    y -= dims.image_offset_y + DRAW_MANAGER.position.y;
    y = Math.floor(y/dims.box_width);

    if(TOOL_MANAGER.cur_selected == "paint"){
      PIXEL_MANAGER.alterPixel(x,y);
      DRAW_MANAGER.draw();
    }else if(TOOL_MANAGER.cur_selected == "erase"){
      PIXEL_MANAGER.erasePixel(x,y);
    }else if(TOOL_MANAGER.cur_selected == "sample"){
      TOOL_MANAGER.sampleColor(x,y);
    }
  
}

var bButtonPressed = false;

let last_x = null;
let last_y = null;

function onmousemove(event){
  if(!bButtonPressed){
    return;
  }

  var x = Math.floor((event.pageX - canvas.offsetLeft)/BOX_WIDTH);
  var y = Math.floor((event.pageY - canvas.offsetTop)/BOX_WIDTH);
  var diff_x = (event.pageX - canvas.offsetLeft);
  var diff_y = (event.pageY - canvas.offsetTop);
  handlemove(diff_x, diff_y);
  
}

function handlemove(diff_x, diff_y){

  if(TOOL_MANAGER.cur_selected == "move" && bButtonPressed){
    if(last_x != null){
      DRAW_MANAGER.move(diff_x-last_x, null);
    }
    last_x = diff_x;
      
    if(last_y != null){
      DRAW_MANAGER.move(null, diff_y-last_y);
      last_y = diff_y;
    }
    last_y = diff_y;
    
  }else{
    last_x = null;
    last_y = null;
  }

}

function onmousedown(event){
  bButtonPressed = true;
}
function onmouseup(event){
  bButtonPressed = false;
  last_x = null;
  last_y = null;
}

function ontouchstart(e){
  bButtonPressed = true;
  e.preventDefault();

  // Check if we need to paint anything
  let touch = e.touches[0];
  let dims = PIXEL_MANAGER.getDimensions();

  var x = touch.pageX - canvas.offsetLeft; // Position on the canvas X
  x -= dims.image_offset_x + DRAW_MANAGER.position.x;
  x = Math.floor(x/dims.box_width);

  var y = touch.pageY - canvas.offsetTop;// Position on the canvas Y
  y -= dims.image_offset_y + DRAW_MANAGER.position.y;
  y = Math.floor(y/dims.box_width);

  if(TOOL_MANAGER.cur_selected == "paint"){
    PIXEL_MANAGER.alterPixel(x,y);
    DRAW_MANAGER.draw();
  }else if(TOOL_MANAGER.cur_selected == "erase"){
    PIXEL_MANAGER.erasePixel(x,y);
  } else if(TOOL_MANAGER.cur_selected == "sample"){
    TOOL_MANAGER.sampleColor(x,y);
  }
}

function ontouchmove(e){
  let touch = e.touches[0];
  let diff_x = (touch.pageX - canvas.offsetLeft);
  let diff_y = (touch.pageY - canvas.offsetTop);
  handlemove(diff_x, diff_y);
  // console.log(touch);
  e.preventDefault();
}
function ontouchend(e){
  // var touch = e.touches[0];
  bButtonPressed = false;
  e.preventDefault();
  last_x = null;
  last_y = null;
}