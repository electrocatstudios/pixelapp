function ToolManager(){
    this.update = ToolManagerUpdate;
    this.scale = 1;
    this.cur_selected = "paint";
    this.cur_selected_layer = "color";

    this.clickMenuItem = ToolManagerClickMenuItem;

    this.refreshFrameButtons = ToolManagerRefreshFrameButtons;

    this.keyDown = ToolManagerKeyDown;

    this.updateColorPicker = ToolManagerUpdateColorPicker;
    this.sampleColor = ToolManagerSampleColor;

    this.color_picker_shown = false;
}

function ToolManagerUpdate(){
    if(this.cur_selected == "view"){
        // Show the view menu
    }
    var val = $('#color_picker').val();
    $('#toolbar_color_show').css({'background-color': val});
    $('#toolbar_color_sample').css({'background-color': val});
}

function ToolManagerClickMenuItem(item_name){
    // Make sure the buttons aren't selected
    $('#toolbar_button_paint').removeClass("toolbar_item_selected");
    $('#toolbar_button_erase').removeClass("toolbar_item_selected");
    $('#toolbar_button_move').removeClass("toolbar_item_selected");
    $('#toolbar_button_view').removeClass("toolbar_item_selected");
    $('#toolbar_button_color').removeClass("toolbar_item_selected");
    $('#toolbar_button_sample').removeClass("toolbar_item_selected");
    
    if(item_name == "zoom_plus"){
        this.scale += 1;
        if(this.scale > 4){
            this.scale = 4;
        }  
        $('#toolbar_color_picker').hide(400);
        this.color_picker_shown = false;
        return; 
    }else if(item_name == "zoom_minus"){
        this.scale -= 1;
        if(this.scale < 1){
            this.scale = 1;
        }
        $('#toolbar_color_picker').hide(400);
        this.color_picker_shown = false;
        return;
        
    }else if(item_name=='color'){
        if(this.color_picker_shown){
            $('#toolbar_color_picker').hide(400);
            this.color_picker_shown = false;
        }else{
            $('#toolbar_button_color').addClass("toolbar_item_selected");
            $('#toolbar_color_picker').show(400);
            position_color_bar_picker();
            this.updateColorPicker();
            this.color_picker_shown = true;
        }
        
        return;
    }else{
        $('#toolbar_color_picker').hide(400);
        this.color_picker_shown = false;
        this.cur_selected = item_name;
    }
    $('#toolbar_button_' + this.cur_selected).addClass("toolbar_item_selected");
}

function ToolManagerRefreshFrameButtons(){
    var num_buttons = PIXEL_MANAGER.getNumberFrames();
    var output = "";
    for(var i=0;i<num_buttons;i++){
      var index = i+1;
      
      output += "<button class='frame_button "; 
      if(i == DRAW_MANAGER.cur_frame){
        output += "selected_button"
      }
      output +="' onclick='load_frame(" + index + ")'>" + index +"</button>"
    }  
    output += "<button class='frame_button' onclick='new_frame()'>+</button>";
    $('#frame_buttons').html(output);
  
}

function ToolManagerKeyDown(key){
    
    if(TOOL_MANAGER.cur_selected=="move"){
        if(key=="A"){
            let dims = PIXEL_MANAGER.getDimensions();
            DRAW_MANAGER.move(dims.box_width * -1, null);
            return true;
        }
        if(key=="D"){
            let dims = PIXEL_MANAGER.getDimensions();
            DRAW_MANAGER.move(dims.box_width, null);
            return true;
        }
        if(key=="W"){
            let dims = PIXEL_MANAGER.getDimensions();
            DRAW_MANAGER.move(null, dims.box_width * -1);
            return true;
        }
        if(key=="S"){
            let dims = PIXEL_MANAGER.getDimensions();
            DRAW_MANAGER.move(null, dims.box_width);
            return true;
        }
        if(key == "O"){
            DRAW_MANAGER.resetOrigin();
            return true;
        }
    }
    
    return false
}

function ToolManagerUpdateColorPicker(){
    // Set the value of each color bar to the correct value
    let val =null;
    if(this.cur_selected_layer == "color"){
        val = $('#color_picker').val();
        // console.log(val)
    }else if(this.cur_selected_layer == "background"){
        val = DRAW_MANAGER.background_color;
        // console.log(val)
    }

    let alpha = null;
    if(this.cur_selected_layer == "shader"){
        $('#color_bar_red').hide();
        $('#color_bar_green').hide();
        $('#color_bar_blue').hide();

        $('#color_bar_input_red').hide();
        $('#color_bar_input_green').hide();
        $('#color_bar_input_blue').hide();

        alpha = DRAW_MANAGER.alpha_channel_shader;
    }else{

        $('#color_bar_red').show();
        $('#color_bar_green').show();
        $('#color_bar_blue').show();

        $('#color_bar_input_red').show();
        $('#color_bar_input_green').show();
        $('#color_bar_input_blue').show();

        let red = parseInt(val.substring(1,3),16);
        let green = parseInt(val.substring(3,5),16);
        let blue = parseInt(val.substring(5,7),16);
        
        $('#color_bar_input_red').val(red);
        $('#color_bar_input_green').val(green);
        $('#color_bar_input_blue').val(blue);
        
        let diff_red = 100 - parseInt((red/255) * 100);
        let gradient_red = 'linear-gradient(to bottom, #ffffff 0%, #ffffff ' + diff_red + '%,#ff0000 ' + (diff_red+1) +'%,#ff0000 100%)';
        $('#color_bar_red').css({'background': gradient_red})
    
        let diff_green = 100 - parseInt((green/255) * 100);
        let gradient_green = 'linear-gradient(to bottom, #ffffff 0%, #ffffff ' + diff_green + '%,#00ff00 ' + (diff_green+1) + '%,#00ff00 100%)';
        $('#color_bar_green').css({'background': gradient_green})
        
        let diff_blue = 100 - parseInt((blue/255) * 100)
        let gradient_blue = 'linear-gradient(to bottom, #ffffff 0%, #ffffff ' + diff_blue + '%,#0000ff ' + (diff_blue+1) + '%,#0000ff 100%)';
        $('#color_bar_blue').css({'background': gradient_blue});
        
        alpha =  DRAW_MANAGER.alpha_channel;
    }
    $('#color_bar_input_alpha').val(alpha);
    
    // Draw the alpha channel
    let perc = alpha * 100;
    let perc_plus = perc +1;
    if(perc == 100){
        perc = 99;
        perc_plus = 100;
    }
    let gray_val = (parseInt( (alpha * 255) )).toString(16);
    let gray_val_plus = (parseInt( (alpha * 255) + 1 )).toString(16);

    if(alpha == 1){
        gray_val_plus = "ff";
    }else if(gray_val_plus.length < 2){
        gray_val_plus = "0" + gray_val_plus;
    }

    if(gray_val.length < 1){
        gray_val = "00";
    }else if(gray_val.length < 2){
        gray_val = "0" + gray_val;
    }

    gray_val = gray_val + gray_val + gray_val;
    gray_val_plus = gray_val_plus + gray_val_plus + gray_val_plus;

    let gradient_alpha = 'linear-gradient(to top, #000000 0%,';
    gradient_alpha += ' #' + gray_val +' ' + parseInt(perc) + '%,';
    gradient_alpha += ' #ff0000 ' + parseInt(perc) + '%,';
    gradient_alpha += ' #ff0000 ' + parseInt(perc_plus) + '%,';
    gradient_alpha += ' #' + gray_val_plus +' ' + parseInt(perc_plus) + '%,';
    gradient_alpha += ' #ffffff 100%)';
    $('#color_bar_alpha').css({'background': gradient_alpha});

}

function ToolManagerSampleColor(x, y){
    let pix = PIXEL_MANAGER.getPixelAt(x, y);
    if(pix == null){
        // console.log("no valid pixel at ",x,y)
        return;
    }
    set_new_color(pix.r, pix.g, pix.b);
    this.updateColorPicker();
}