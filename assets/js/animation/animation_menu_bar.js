$(document).ready(function () {
    $('#applicationMenu').addClass('applicationMenuClosed');

    $('#color_picker').change(function(){
          close_menu();
    })

    $('#tools').show();
    $('#tools_button').addClass('selected_button_black');
    $('#details').hide();
    $('#transforms').hide();

    $('#width_selector').val(window.picture_width);
    $('#height_selector').val(window.picture_height);
    $('#time_length').val(window.animation_length);

    $('#pixel_width_display').val(window.pixel_size);
    $('#backgroundcolor').val("#000000");

});

var applicationMenuShown = false;
var pinned = false;

function close_menu(){
    if(pinned){
        return;
    }
    $('#applicationMenu').removeClass('applicationMenuOpened');
    $('#applicationMenu').addClass('applicationMenuClosed'); 
    setTimeout(function(){applicationMenuShown = false;}, 400);   
}

function toggle_pinned(){
    pinned = !pinned;
    if(pinned){
        $('#pinned_button').html("&#9746;")
    }else{
        $('#pinned_button').html("&#9744;")
    }
}

function openDrawerMenu() {
    applicationMenuShown = !applicationMenuShown;
    if(applicationMenuShown){
        $('#applicationMenu').removeClass('applicationMenuClosed');
        $('#applicationMenu').addClass('applicationMenuOpened');
    }else{
        close_menu();
        // $('#applicationMenu').removeClass('applicationMenuOpened');
        // $('#applicationMenu').addClass('applicationMenuClosed');
    }
    
}

function hide_all(){
    $('#tools').hide();
    $('#tools_button').removeClass('selected_button_black');
    $('#details').hide();
    $('#details_button').removeClass('selected_button_black');
    $('#transforms').hide();
    $('#transforms_button').removeClass('selected_button_black');
}

function load_tools(){
    hide_all();
    $('#tools').show();
    $('#tools_button').addClass('selected_button_black');
}
function load_details(){
    hide_all();
    $('#details').show();
    $('#details_button').addClass('selected_button_black');
}
function load_transforms(){
    console.log("loading transforms")
    hide_all();
    $('#transforms').show();
    $('#transforms_button').addClass('selected_button_black');
}

function update_animation_details(){
    $('#error').html("");

    var width = $('#width_selector').val();
    var height = $('#height_selector').val();
    var time_len = $('#time_length').val();
    
    var data = {
        width: parseInt(width),
        height: parseInt(height),
        time_length: parseInt(time_len)
    }
    
    var url='/api/animation_details/' + window.animation_id;
    $.ajax({
        url: url,
        type: 'POST',
        dataType: 'json',
        data: JSON.stringify(data),
        contentType: "application/json; charset=utf-8",
        // beforeSend: function (xhr) {
        //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
        // },
        success: function(ret){
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            window.location.reload();
        },
        error: function(ret){
            console.log("ERROR when sending update to size");
            console.log(ret);
        }
    })
}