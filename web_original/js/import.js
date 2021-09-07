let filename = null;
let filedata = null;
let image_id = null;
let token = null;

$(document).ready(function(){

    token = getToken();
    if(token == null){
        window.location.href='/login?ret=pixelapp%newfromimage%2f' + window.pixel_id;
        return;
    }

    hide_all_pages();
    $('#page1').show();

    $('#input_file').on('change', function(evt){
        let fn = $('#input_file').val();
        
        if(fn != ""){
            filename = fn;
            filedata = this.files[0];
        }
    })
});

function hide_all_pages(){
    $('#page1').hide();
    $('#page2').hide();
    $('#page3').hide();
    $('#page4').hide();
}

function next_page(page_num){
    $('#error').html("");
    
    if(page_num == 2){
        if(filename == null){
            $('#error').html("Filename cannot be null");
            return;
        }
        send_file();
        return;
    }else if(page_num == 2.5){
        hide_all_pages();
        $('#preview_image').attr('src', '/pixelapp/importimage/' + image_id);
        update_image_details();
        $('#page2').show();
        return;
    }else if(page_num == 3){
        // Send all the data up
        finalize_data();
    }

    hide_all_pages();
    $('#page' + page_num).show();
}

function prev_page(page_num){
    hide_all_pages();
    $('#page' + page_num).show();
}

function send_file(){
    $('#error').html("");

    let formData = new FormData();
    formData.append("pixel_image", filedata);
    let url = "/api/pixelapp/importimage";

    $.ajax({
        url: url,
        type: 'POST',
        data: formData,
        contentType: false,
        processData: false,
        dataType: 'json',
        beforeSend: function (xhr) {
            xhr.setRequestHeader ("Authorization", "Bearer " + token);
        },
        success: function(ret){
            // console.log(ret)
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            image_id = ret.imageid;
            next_page(2.5);
        },
        error: function(ret){
            console.log("Error uploading the image");
            console.log(ret);
        }
    });
}

function update_image_details(){
    let url ='/api/pixelapp/importimage/' + image_id;
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        beforeSend: function (xhr) {
            xhr.setRequestHeader ("Authorization", "Bearer " + token);
        },
        success: function(ret){
            // console.log(ret)
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            $('#width_info').html("Width: " + ret.width + "px");
            $('#height_info').html("Height: " + ret.height + "px"); 
            $('#target_width_value').val(ret.width); 
            $('#target_height_value').val(ret.height);
        },
        error: function(ret){
            console.log("Error uploading the image");
            console.log(ret);
        }
    });
}

function finalize_data(){
    let targetwidth = parseInt($('#target_width_value').val());
    let targetheight = parseInt($('#target_height_value').val());
    let name = $('#pixel_name').val();
    let desc = $('#pixel_description').val();
    let pixelwidth = $('#pixel_width').val();
    pixelwidth = Math.max(parseInt(pixelwidth), 1);
    
    let data = {
        targetwidth: targetwidth,
        targetheight: targetheight,
        endx: targetwidth,
        endy: targetheight,
        name: name,
        description: desc,
        picid: image_id,
        pixelwidth: pixelwidth
    }

    let url = "/api/pixelapp/import/finalize";
    console.log(data);
    $.ajax({
        url: url,
        type: 'POST',
        data: JSON.stringify(data),
        dataType: 'json',
        beforeSend: function (xhr) {
            xhr.setRequestHeader ("Authorization", "Bearer " + token);
        },
        success: function(ret){
            // console.log(ret)
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            // Redirect to the pixel page
            window.location.href='/pixelapp/' + ret.pixelid;
            
        },
        error: function(ret){
            console.log("Error uploading the image");
            console.log(ret);
        }
    });
}