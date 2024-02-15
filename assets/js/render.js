// let token = null;
let number_colors = 0;
$(document).ready(function() {
    "api" / "details" / String
    let url = "/api/details/" + window.pixel_id;
    console.log(url);
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret) {
            if(ret.status != "ok"){
                console.log(ret.message);
                $('#error').html(ret.message);
                return;
            }
            console.log(ret);
            window.picture_width = ret.width;
            window.picture_height = ret.height;
            window.pixel_size = ret.pixelwidth;
            window.pixel_name = ret.name;
            load_page();
        },
        error: function(err) {
            console.log("Error getting rendering data on load");
            console.log(err);
        }

    });
});

function load_page(){
    // token = getToken();
    // if(token == null){
    //     window.location.href='/login?ret=pixelapp%2frender%2f' + window.pixel_id;
    //     return;
    // }
    $('#color_values').hide();
    $('#preview_display').attr('src', '/img/spritesheet/' + window.pixel_id  )
    let url = "/api/info/" + window.pixel_id;
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        // beforeSend: function (xhr) {
        //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
        // },
        success: function(ret){
            console.log(ret);
            if(ret.status != "ok"){
                $('#error').html(ret.message);
                return;
            }
            let output = ""
            for(let i=0;i<ret.framecount;i++){
                output += "<option value='" + i + "'>Frame " + (i+1) + "</option>";
            }
            $('#framelist').html(output);
            updateFramePreview();

            let colors_output = "";
            number_colors = ret.colors.length;
            for(let i=0;i<ret.colors.length;i++){
                var col = ret.colors[i];
                // console.log(col);
                colors_output += "<span class='render_color_sample' style='background-color:#" + col +"' id='demopanel_" + col + "'></span>"
                colors_output += " =&gt; ";
                colors_output += "<input type='color' value='#" + col + "' id='newcol_" + col  + "'>";
                colors_output += "<br>";
            }   
            colors_output += "<button onclick='closeSubColorArea()' style='margin-right: 40px;'>Close</button>";
            colors_output += "<button onclick='getSubColorSpritesheet()'>Render with Colors</button>"
            // console.log(colors_output);
            $('#color_values').html(colors_output);

        }
    })

    $('#framelist').on('change', function(){
        updateFramePreview();
    })
    $('#rotation').on('change', function(){
        updateFramePreview();
    })
    $('#flip').on('change', function(){
        updateFramePreview();
    })

}

function updateFramePreview(){
    let val = $('#framelist').val();
    let angle = $('#rotation').val();
    let flip = ($('#flip:checked').length > 0);
    let url = "/img/render/" + window.pixel_id + "/" + val + "/forward/" + angle + "/" + flip;
    $('#single_frame_preview').attr('src', url);
}

function getPixelSpriteSheet(){
    let include_reverse = ($('#include_reverse:checked').length > 0);
    if(include_reverse){
        window.location.href='/img/spritesheet/' + window.pixel_id + "?render_type=forward";
    } else {
        window.location.href='/img/spritesheet/' + window.pixel_id + "?render_type=both";
    }
}

function getPixelGif() {
    let include_reverse = ($('#include_reverse:checked').length > 0);

    if(include_reverse){
        callUrlWithColorSubs('/img/gif/' + window.pixel_id + "?render_type=both&", true);
    } else {
        callUrlWithColorSubs('/img/gif/' + window.pixel_id + "?render_type=forward&", true);
    }
}

function renderSingleFrame(){
    let include_reverse = ($('#include_reverse:checked').length > 0);

    let flip = ($('#flip:checked').length > 0);

    let val = $('#framelist').val();
    let angle = $('#rotation').val();
    if(include_reverse){
        window.location.href="/img/render/" + window.pixel_id + "/" + val + "/both/" + angle + "/" + flip;
    }else{
        window.location.href="/img/render/" + window.pixel_id + "/" + val + "/forward/" + angle + "/" + flip;
    }
    
}

function getFileDownload(){
    // eg. http://localhost:8081/api/5b7f7738-101e-493b-a427-fdf88f2f29fd
    let filename = $('#filename').val();
    $('#filenameerror').html("");
    if(filename == undefined || filename == null || filename == ""){
        $('#filenameerror').html("Filename cannot be blank");
        return;
    }
    filename = filename + ".json";

    let url = "/api/saveasfile/" + window.pixel_id;
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        // beforeSend: function (xhr) {
        //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
        // },
        success: function(ret){
            let a = document.createElement('a');
            a.setAttribute('href', 'data:text/plain;charset=utf-8,'+encodeURIComponent(JSON.stringify(ret)));
            a.setAttribute('download', filename);
            a.click()
        },
        error: function(ret){
            console.log("ERROR while getting data to save");
            console.log(ret);
        }
      });
}

function getSubColorPage(){
    // TODO: Show the color substitutions section
    $('#color_values').show();
    // window.location.href='/pixelapp/rendersub/'
}

function closeSubColorArea(){
    $('#color_values').hide();
}

function callUrlWithColorSubs(url, new_window) {
    let query = ""
    let bFoundOne = false;
    let el_count = 0;
    $('#color_values span').each(function(ind, el){
        
        let id_name = $(el).attr('id');
        let col_val = id_name.split("_")[1]
        let inp_val = $('#newcol_' + col_val).val();
        if( ("#" + col_val) != inp_val){
            if(bFoundOne){
                query += "&"
            }
            bFoundOne = true;
            query += col_val + "=" + inp_val.split("#")[1]
        }
        el_count += 1;
        if(el_count == number_colors){
            if(new_window){
                window.open(url + query, '_blank');
            } else {
                window.location.href= url + query;
            }
        }
        
    })
}

function getSubColorSpritesheet(){
    // Get the color substitutes
    callUrlWithColorSubs('/img/spritesheet/' + window.pixel_id + "?");
}