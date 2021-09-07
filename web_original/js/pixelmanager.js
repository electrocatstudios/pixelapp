function PixelManager(){
    this.pixel_id = window.pixel_id;

    this.update = PixelManagerUpdate;
    this.loadData = PixelManagerLoadData;
    this.setPixel = PixelManagerSetPixel;

    this.setShader = PixelManagerSetShader;
    this.erasePixel = PixelManagerErasePixel;
    this.getFrame = PixelManagerGetFrame;
    this.getFrameShader = PixelManagerGetFrameShader;

    this.getNumberFrames = PixelManagerGetNumberFrames;
    this.newFrame = PixelManagerNewFrame;
    this.getPixelData = PixelManagerGetPixelData;
    this.getShaderData = PixelManagerGetShaderData;

    this.getDimensions = PixelManagerGetDimensions;

    this.saved_image_data = [];
    this.shown_image_data = [];

    this.shader_layer = [];

    this.max_frame = 0;

    this.alterPixel = PixelManagerAlterPixel;
    this.getPixelAt = PixelManagerGetPixelAt;

    this.loadData(this.pixel_id);
}

function PixelManagerLoadData(pixel_id){
    let numX = window.picture_width/window.pixel_size;
    let numY = window.picture_height/window.pixel_size;
    this.saved_image_data[0] = [];
    for(let y=0;y<numY;y++){
        this.saved_image_data[0].push([]);
        // console.log(this.saved_image_data);
        for(let x=0;x<numX;x++){
            this.saved_image_data[0][y].push(null);
        }
    }

    var url = "/api/pixelapp/" + pixel_id;
    $.ajax({
      url: url,
      type: 'GET',
      dataType: 'json',
      beforeSend: function (xhr) {
          xhr.setRequestHeader ("Authorization", "Bearer " + token);
      },
      success: function(ret){
        // console.log(ret)
;
        if(ret.status != "ok"){
            // console.log(ret.message);
            $('#error').html(ret.message);
            return;
        }
        
        let _box_width = BOX_WIDTH * TOOL_MANAGER.scale;
        
        var max_frame = 0;
        for(var i=0;ret.pixels != null && i<ret.pixels.length;i++){
            var p = ret.pixels[i];
            
            // setPixel(p.x, p.y, p.r, p.g, p.b, p.alpha, 1, p.layer, p.frame);
            if(p.frame > PIXEL_MANAGER.max_frame){
                PIXEL_MANAGER.max_frame = p.frame;
            }
            
            if(PIXEL_MANAGER.saved_image_data[p.frame] == undefined){
                PIXEL_MANAGER.saved_image_data[p.frame] = null;
            }
            if(PIXEL_MANAGER.shader_layer[p.frame] == undefined){
                PIXEL_MANAGER.shader_layer[p.frame] = null;
            }
            PIXEL_MANAGER.setPixel(p.x, p.y, p.r, p.g, p.b, p.alpha, p.frame);
        }
        
        TOOL_MANAGER.refreshFrameButtons();
      },
      error: function(ret){
          console.log("ERROR while getting saved data");
          console.log(ret);
      }
    });

    let shader_url = '/api/pixelapp/shader/' + pixel_id;
    $.ajax({
        url: shader_url,
        type: 'GET',
        dataType: 'json',
        beforeSend: function (xhr) {
            xhr.setRequestHeader ("Authorization", "Bearer " + token);
        },
        success: function(ret){
          if(ret.status != "ok"){
              // console.log(ret.message);
              $('#error').html(ret.message);
              return;
          }
          let _box_width = BOX_WIDTH * TOOL_MANAGER.scale;
      
          var max_frame = 0;
          for(var i=0;ret.shaders != null && i<ret.shaders.length;i++){
              var s = ret.shaders[i];
              
              // setPixel(p.x, p.y, p.r, p.g, p.b, p.alpha, 1, p.layer, p.frame);
              if(s.frame > PIXEL_MANAGER.max_frame){
                  PIXEL_MANAGER.max_frame = p.frame;
              }

              if(PIXEL_MANAGER.shader_layer[s.frame] == undefined){
                  PIXEL_MANAGER.shader_layer[s.frame] = null;
              }
              if(PIXEL_MANAGER.saved_image_data[s.frame] == undefined){
                PIXEL_MANAGER.saved_image_data[s.frame] = null;
              }
              PIXEL_MANAGER.setShader(s.x, s.y, s.r, s.g, s.b, s.alpha, s.frame);
          }
          TOOL_MANAGER.refreshFrameButtons();
        },
        error: function(ret){
            console.log("ERROR while getting saved data");
            console.log(ret);
        }
      });
}

function PixelManagerUpdate(){

}

function PixelManagerSetPixel(x, y, r, g, b, a, frame){
    if(this.saved_image_data[frame] == null){
        let pix_width = parseInt(window.picture_width/window.pixel_size);
        let pix_height = parseInt(window.picture_height/window.pixel_size);
        
        this.saved_image_data[frame] = new Array(pix_height).fill(null);
        for(i=0;i<pix_height;i++){
            this.saved_image_data[frame][i] = new Array(pix_width).fill(null);;
        }
    }
    
    var pixel = {
        r: r,
        g: g,
        b: b,
        alpha: a
    }
    this.saved_image_data[frame][y][x] = pixel;
}

function PixelManagerSetShader(x, y, r, g, b, a, frame){
    if(this.shader_layer[frame] == null){
        let pix_width = parseInt(window.picture_width/window.pixel_size);
        let pix_height = parseInt(window.picture_height/window.pixel_size);
        
        this.shader_layer[frame] = new Array(pix_height).fill(null);
        for(i=0;i<pix_height;i++){
            this.shader_layer[frame][i] = new Array(pix_width).fill(null);;
        }
    }
    var shader = {
        r: r,
        g: g,
        b: b,
        alpha: a
    }
    this.shader_layer[frame][y][x] = shader;
}

function PixelManagerGetFrame(frame){
    if(this.saved_image_data[frame] == undefined || this.saved_image_data[frame] == null){
        return [];
    }
    // let pix_width = parseInt(window.picture_width/window.pixel_size);
    // let pix_height = parseInt(window.picture_height/window.pixel_size);
    let pix_height = this.saved_image_data[frame].length;
    let pix_width = this.saved_image_data[frame][0].length;

    // console.log(pix_width, pix_height);

    var ret = new Array(pix_height).fill(null);
    for(i=0;i<pix_height;i++){
        ret[i] = new Array(pix_width).fill(null);;
    }
    
    for(var y=0;y<this.saved_image_data[frame].length;y++){
        for(var x=0;x<this.saved_image_data[frame][y].length;x++){
            
            let pix = this.saved_image_data[frame][y][x];
            // console.log(pix);
            if(pix != null){
                let nxt = {
                    r: pix.r,
                    g: pix.g,
                    b: pix.b,
                    alpha: pix.alpha
                }
                // console.log(nxt)
                ret[y][x] = nxt;
            }
            // console.log(pix);
          
        }
    }
    return ret;
}

function PixelManagerGetFrameShader(frame){
    if(this.shader_layer[frame] == undefined || this.shader_layer[frame] == null){
        return [];
    }
    let pix_height = this.shader_layer[frame].length;
    let pix_width = this.shader_layer[frame][0].length;

    var ret = new Array(pix_height).fill(null);
    for(i=0;i<pix_height;i++){
        ret[i] = new Array(pix_width).fill(null);;
    }
    
    for(var y=0;y<this.shader_layer[frame].length;y++){
        for(var x=0;x<this.shader_layer[frame][y].length;x++){
            
            let pix = this.shader_layer[frame][y][x];
            if(pix != null){
                let nxt = {
                    r: pix.r,
                    g: pix.g,
                    b: pix.b,
                    alpha: pix.alpha
                }
                ret[y][x] = nxt;
            }
        }
    }
    return ret;
}


function PixelManagerAlterPixel(x,y){
    var color = $('#color_picker').val();
    var col_obj = hexToRgb(color);
    
    if(y > this.saved_image_data[0].length -1 || x > this.saved_image_data[0][0].length-1){
        // Out of bounds
        console.log("Out of bounds")
        return;
    }
    if(DRAW_MANAGER.view_list.shader){
        // Drawing to shader layer because it's visible
        // TODO: Respect the color selected rather than hard-coding black 
        this.setShader(x, y, 0, 0, 0, DRAW_MANAGER.alpha_channel_shader, DRAW_MANAGER.cur_frame);
    }else if(DRAW_MANAGER.view_list.foreground) {
        // Draw to foreground
        this.setPixel(x, y, col_obj.r, col_obj.g, col_obj.b, DRAW_MANAGER.alpha_channel, DRAW_MANAGER.cur_frame);
    }
}

function PixelManagerGetPixelAt(x,y){
    if(DRAW_MANAGER.cur_frame >= this.saved_image_data.length){
        return null;
    }
    if(y > this.saved_image_data[DRAW_MANAGER.cur_frame].length -1 || x > this.saved_image_data[DRAW_MANAGER.cur_frame][0].length-1){
        return null;
    }
    let ret = this.saved_image_data[DRAW_MANAGER.cur_frame][y][x];
    if(ret == undefined){
        ret = null;
    }
    return ret;
}

function PixelManagerErasePixel(x, y){
    this.setPixel(x, y, 0, 0, 0, 0, DRAW_MANAGER.cur_frame);
}

function PixelManagerGetNumberFrames(){
    return this.saved_image_data.length;
}

function PixelManagerNewFrame(){
    var pix_width = parseInt(window.picture_width/window.pixel_size);
    var pix_height = parseInt(window.picture_height/window.pixel_size);
    
    // Set up the new line
    var new_line = new Array(pix_height).fill(null);
    for(i=0;i<pix_height;i++){
        new_line[i] = new Array(pix_width).fill(null);;
    }

    // Copy previous selected frame
    for(var y=0;y<new_line.length;y++){
        for(var x=0;x<new_line[y].length;x++){
            var p = this.saved_image_data[DRAW_MANAGER.cur_frame][y][x];
            if(p != null){
                var pixel = {
                    r: p.r,
                    g: p.g,
                    b: p.b,
                    alpha: p.alpha
                }
                new_line[y][x] = pixel;
            } 
        }
    }

    this.saved_image_data.splice(DRAW_MANAGER.cur_frame+1,0,new_line);

    // Move to the newly created frame
    DRAW_MANAGER.cur_frame += 1;
}

function PixelManagerGetDimensions(){
    let _box_width = BOX_WIDTH * TOOL_MANAGER.scale;
    if(PIXEL_MANAGER.saved_image_data == undefined || PIXEL_MANAGER.saved_image_data[0] == undefined){
        return;
    }
    let max_y = PIXEL_MANAGER.saved_image_data[0].length;
    let max_x = PIXEL_MANAGER.saved_image_data[0][0].length;

    let ret = {
        box_width: _box_width,
        max_y: max_y,
        max_x: max_x,
        act_width: max_x * _box_width,
        act_height: max_y * _box_width,
        offset_x: null,
        offset_y: null,
        image_offset_x: null,
        image_offset_y: null
    }

    ret.offset_x = Math.max((GAME_SIZE.x/2) - (ret.act_width/2) - ret.box_width, 0);
    ret.offset_y = Math.max( (GAME_SIZE.y/2) - (ret.act_height/2) - ret.box_width, 0)
    
    // Because actual image starts one box width over
    ret.image_offset_x = ret.offset_x + ret.box_width;
    ret.image_offset_y = ret.offset_y + ret.box_width;
    
    return ret;
}

function PixelManagerGetPixelData(){
    let ret = []
    for(let f=0;f<this.saved_image_data.length;f++){
        for(let y=0;y<this.saved_image_data[f].length;y++){
            for(let x=0;x<this.saved_image_data[f][y].length;x++){
                 if(this.saved_image_data[f][y][x]!=null){
                    let p = this.saved_image_data[f][y][x];
                    let alpha = parseFloat(p.alpha);
                    if(alpha > 1.0){
                        alpha = 1.0;
                    }
                    let nxt = {
                        x: x,
                        y: y,
                        r: p.r,
                        g: p.g,
                        b: p.b,
                        alpha: alpha,
                        // layer: 0,
                        frame: f
                    }
                    // console.log(p.alpha);
                    ret.push(nxt);
                }
            }
        }
    }
   
    return ret;
}

function PixelManagerGetShaderData(){
    let ret = [];
    for(let f=0;f<this.shader_layer.length;f++){
        for(let y=0;this.shader_layer[f] != null && y<this.shader_layer[f].length;y++){
            for(let x=0;this.shader_layer[f][y] != null && x<this.shader_layer[f][y].length;x++){
                 if(this.shader_layer[f][y][x]!=null){
                    let s = this.shader_layer[f][y][x];
                    let alpha = parseFloat(s.alpha);
                    if(alpha > 1.0){
                        alpha = 1.0;
                    }
                    let nxt = {
                        x: x,
                        y: y,
                        r: s.r,
                        g: s.g,
                        b: s.b,
                        alpha: alpha,
                        // layer: 0,
                        frame: f
                    }
                    ret.push(nxt);
                }
            }
        }
    }
    return ret;
}