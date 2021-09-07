function Location(_x, _y){
    this.x = _x;
    this.y = _y;
}
  
function setPixel(x, y, r, g, b, a, scale, layer, frame) {
    /*image grid - the display grid*/
    if(saved_image_data!= null){
        if(saved_image_data[frame] == null){
            var pix_width = parseInt(window.picture_width/window.pixel_size);
            var pix_height = parseInt(window.picture_height/window.pixel_size);
            
            saved_image_data[frame] = new Array(pix_height).fill(null);
            for(i=0;i<pix_height;i++){
                saved_image_data[frame][i] = new Array(pix_width).fill(null);;
            }
        }
        
        var pixel = {
            r: r,
            g: g,
            b: b,
            alpha: a
        }
        saved_image_data[frame][y][x] = pixel;
    }

    if(frame == cur_frame){
        // console.log("Updating image data");
        update_image_data(x, y, r, g, b, a, scale, layer, frame);
    }else{
        // console.log(frame, cur_frame);
    }
    
}

function update_image_data(x, y, r, g, b, a, scale, layer, frame){
    var index_grid = (x + y * image_data_grid.width) * 4;
    image_data_grid.data[index_grid+0] = r;
    image_data_grid.data[index_grid+1] = g;
    image_data_grid.data[index_grid+2] = b;
    image_data_grid.data[index_grid+3] = a;
  
    /*image data - the preview*/
    var img_w = image_data.width * 4;
    index = (x*scale*4) + (y * scale * img_w);
  
    /* j is the horizontal repetitions
       i is the vertical */
  
    for(var i = 0; i < scale; i++){
        for (var j = 0; j < scale; j++){
            
            image_data.data[index+0+(i*img_w)+(j*4)] = r;
            image_data.data[index+1+(i*img_w)+(j*4)] = g;
            image_data.data[index+2+(i*img_w)+(j*4)] = b;
            image_data.data[index+3+(i*img_w)+(j*4)] = a;
        }
    }
}

function getPixels(){
    if(saved_image_data == null){
        console.log("ERROR trying to get saved_image_data but it's null");
        return;
    }
    // var step_size = parseInt(window.pixel_size/2);
    var pix_width = parseInt(window.picture_width/window.pixel_size);
    var pix_height = parseInt(window.picture_height/window.pixel_size);
    var ret = []
    for(var f=0;f<saved_image_data.length;f++){
        for(var y=0;y<pix_height;y++){
            for(var x=0;x<pix_width;x++){
                 if(saved_image_data[f][y][x]!=null){
                    var p = saved_image_data[f][y][x];
                    var nxt = {
                        x: x,
                        y: y,
                        r: p.r,
                        g: p.g,
                        b: p.b,
                        alpha: p.alpha,
                        layer: 0,
                        frame: f
                    }
                    ret.push(nxt);
                }
            }
        }
    }
   
    return ret;
}
  
function getPixelColor(imageData, x, y){
    index = (x + y * imageData.width) * 4;
    return {'r': imageData.data[index+0], 'g':imageData.data[index+1],
            'b': imageData.data[index+2], 'a':imageData.data[index+3]};
}
  
function hexToRgb(hex) {
    var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result ? {
      r: parseInt(result[1], 16),
      g: parseInt(result[2], 16),
      b: parseInt(result[3], 16)
    } : null;
}
  
function readCookie(name) {
    var nameEQ = encodeURIComponent(name) + "=";
    var ca = document.cookie.split(';');
    for (var i = 0; i < ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) === ' ') c = c.substring(1, c.length);
        if (c.indexOf(nameEQ) === 0) return decodeURIComponent(c.substring(nameEQ.length, c.length));
    }
    return null;
}

var _TOKEN_COOKIE_NAME = "ecs_gsv_token";
function eraseCookie(name) {
    createCookie(name, "", -1);
}

function setToken(token){
    createCookie(_TOKEN_COOKIE_NAME, token, 7);
}
function getToken(){
    return readCookie(_TOKEN_COOKIE_NAME);
}

function logout(redirect){
    eraseCookie(_TOKEN_COOKIE_NAME);
    if(redirect){
        window.location.href='/';
    }
}