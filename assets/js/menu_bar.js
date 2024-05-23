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
    $('#pixel_width_display').val(window.pixel_size);
    $('#backgroundcolor').val("#000000");

    // Show the selected collection
    var url = "/api/collection";
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret){
            if(ret.status != "ok"){
                $('#error').html(ret.message);
                return;
            }
            var output = "<select id='sel_collection'><option>--All--</option>";
            for(var i=0;i<ret.collections.length;i++){
                var c = ret.collections[i];
                output += "<option value='" + c.id + "'";
                if(c.id == window.collection_id) {
                    output += " selected";
                }

                if(c.name === ""){
                    output += ">[None]</option>"
                } else {
                    output += ">" + c.name + "</option>"
                }
            }
            output += "</select>";
            $('#sel_coll').html(output);
        },
        error: function(ret){
            console.log("Error getting saved pixels")
            console.log(ret)
        }
    })
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

function update_details(){
    $('#error').html("");

    var width = $('#width_selector').val();
    var height = $('#height_selector').val();
    var collection = $('#sel_collection').val();
    
    var data = {
        width: parseInt(width),
        height: parseInt(height)
    }

    if(collection !== undefined && collection !== null && collection !== 0){
        data.collection = parseInt(collection);
    }
    
    var url='/api/size/' + window.pixel_id;
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