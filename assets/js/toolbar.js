$(document).ready(function(){
    var screen_width = $(document).width();
    if (GAME_SIZE.mobile) {
        $('#pixel_toolbar').css({'right': 5 + "px", "top":"70px" });
    } else {
        $('#pixel_toolbar').css({'left': ((screen_width/2) + (GAME_SIZE.x/2) + 10) + "px", "top":"70px" });
    }
    $('#pixel_zoom_buttons').css({'left': (screen_width/2) + ((GAME_SIZE.x/2) - 105) + 'px','top': (GAME_SIZE.y + 75) + "px"});
   
    // View buttons
    var view_button_left = (($(document).width()/2) - (GAME_SIZE.x/2) - 50) ;
    if (GAME_SIZE.mobile) {
        $('#toolbar_view_select').css({'left': 5 + 'px', 'top': 70 + "px"});
    } else {
        $('#toolbar_view_select').css({'left': view_button_left + 'px', 'top': 70 + "px"});
    }
    
    $('#color_bar_red').click(function(evt){
        let posY = evt.pageY - $(this).offset().top;
        let val = parseInt(((200-posY) / 200) * 255);
        $('#color_bar_input_red').val(val);
        
        set_new_color(val, null, null);
    });
    $('#color_bar_input_red').on('change', function(evt){
        let val = $('#color_bar_input_red').val();
        
        set_new_color(val, null, null);
    });

    $('#color_bar_green').click(function(evt){
        let posY = evt.pageY - $(this).offset().top;
        let val = parseInt(((200-posY) / 200) * 255);
        $('#color_bar_input_green').val(val);

        set_new_color(null, val, null);
    });
    $('#color_bar_input_green').on('change', function(evt){
        let val = $('#color_bar_input_green').val();
        set_new_color(null, val, null);
    });

    $('#color_bar_blue').click(function(evt){
        let posY = evt.pageY - $(this).offset().top;
        let val = parseInt(((200-posY) / 200) * 255);
        $('#color_bar_input_blue').val(val);

        set_new_color(null, null, val);
    });
    $('#color_bar_input_blue').on('change', function(evt){
        let val = $('#color_bar_input_blue').val();
        set_new_color(null, null, val);
    });

    $('#color_bar_alpha').click(function(evt){
        let posY = evt.pageY - $(this).offset().top;
        let val = (200-posY) / 200; // Range between 0 - 1
        val = val.toFixed(2);
        if(val > 0.95){
            val = 1;
        }
        if(val < 0.05){
            val = 0;
        }
        $('#color_bar_input_alpha').val(val);
        set_new_alpha(val);
    });
    $('#color_bar_input_alpha').on('change', function(evt){
        let val = $('#color_bar_input_alpha').val();
        set_new_alpha(val);
    });

});

function set_new_color(red, green, blue){
    let col = null;
    
    if(TOOL_MANAGER.cur_selected_layer == "color"){
        col = $('#color_picker').val();
    }else if(TOOL_MANAGER.cur_selected_layer == "background"){
        col = DRAW_MANAGER.background_color;
    }
    if(col == null){
        console.log("Error: color is null");
        return;
    }
    
    if(red == null){
        red = parseInt(col.substring(1,3), 16);
    }else{
        red = parseInt(red);
    }
    if(green == null){
        green = parseInt(col.substring(3,5),16);
    }else{
        green = parseInt(green);
    }
    
    if(blue == null){
        blue = parseInt(col.substring(5,7),16);
    }else{
        blue = parseInt(blue);
    }
    
    if(red < 10){
        red = "0" + red.toString(16);
    }else{
        red = red.toString(16);
    }
    
    if(green < 10){
        green = "0" + green.toString(16);
    } else{
        green = green.toString(16);
    } 
    if(blue < 10){
        blue = "0" + blue.toString(16);
    } else{
        blue = blue.toString(16);
    }
    
    if(TOOL_MANAGER.cur_selected_layer == "color"){
        $('#color_picker').val("#" + red + green + blue);
    }else{
        DRAW_MANAGER.background_color = "#" + red + green + blue;
    }
   
    TOOL_MANAGER.updateColorPicker();
    
}

function set_new_alpha(val){
    if(TOOL_MANAGER.cur_selected_layer == 'shader'){
        DRAW_MANAGER.alpha_channel_shader = val;
    }else{
        DRAW_MANAGER.alpha_channel = val;
    }
    
    TOOL_MANAGER.updateColorPicker();
}

// Support function should be handled by the tool manager
function click_menu_item(item_name){
    TOOL_MANAGER.clickMenuItem(item_name);
    // TODO: Display Icon instead of TEXT
    if(item_name == "paint"){
        $('#toolbar_reveal').html("<img src=\"/img/icons/paint.png\">");
        $('#toolbar_reveal').addClass('toolbar_item_selected');
    }else if(item_name == "fill"){
        $('#toolbar_reveal').html("<img src=\"/img/icons/fill.png\">");
        $('#toolbar_reveal').addClass('toolbar_item_selected');
    }
}

function hover_menu_item(item_name){
    let prompt_val = item_name.charAt(0).toUpperCase() + item_name.slice(1)
    $('#tooltip').html(prompt_val);
}

function hover_menu_exit() {
    $('#tooltip').html("");
}

function position_color_bar_picker(){
    var screen_width = $(document).width();
    $('#toolbar_color_picker').css({'left': (screen_width/2) + ((GAME_SIZE.x/2) - 200 - 80) + 'px','top': 70 + "px"});
}

var paint_tools_showing = false;
function reveal_paint_tools() {
    paint_tools_showing = !paint_tools_showing;
    if(paint_tools_showing){
        $('#paint_tools_selection').removeClass("paint_tools_selection_hide");
        let pos = $('#pixel_toolbar').position();
        $('#paint_tools_selection').css({"top": pos.top + "px", "left": (pos.left - 70) + "px"});
    } else {
        $('#paint_tools_selection').addClass("paint_tools_selection_hide");
    }

}


function select_layer(layer){
    $('#toolbar_select_button_background').removeClass("toolbar_color_button_selected");
    $('#toolbar_select_button_color').removeClass("toolbar_color_button_selected");
    $('#toolbar_select_button_shader').removeClass("toolbar_color_button_selected");
    
    $('#toolbar_select_button_' + layer).addClass("toolbar_color_button_selected");
    TOOL_MANAGER.cur_selected_layer = layer;
    TOOL_MANAGER.updateColorPicker();
}

function toggle_view_button(btn){
    DRAW_MANAGER.toggleView(btn);
}