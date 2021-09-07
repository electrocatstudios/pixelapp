let token = null;
let number_colors = 0;
$(document).ready(function(){
    token = getToken();
    if(token == null){
        window.location.href='/login?ret=pixelapp%2frender%2f' + window.pixel_id;
        return;
    }
    $('#color_values').hide();
    $('#preview_display').attr('src', '/img/pixelapp/' + window.pixel_id  + "/spritesheet")
    let url = "/api/pixelapp/info/" + window.pixel_id;
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        beforeSend: function (xhr) {
            xhr.setRequestHeader ("Authorization", "Bearer " + token);
        },
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

});

function updateFramePreview(){
    let val = $('#framelist').val();
    let angle = $('#rotation').val();
    let flip = ($('#flip:checked').length > 0);
    let url = "/img/pixelapp/render/" + window.pixel_id + "/" + val + "/forward/" + angle + "/" + flip;
    $('#single_frame_preview').attr('src', url);
}

function getPixelSpriteSheet(){
    window.location.href='/img/pixelapp/' + window.pixel_id  + "/spritesheet";
}

function renderSingleFrame(){
    let include_reverse = ($('#include_reverse:checked').length > 0);

    let flip = ($('#flip:checked').length > 0);

    let val = $('#framelist').val();
    let angle = $('#rotation').val();
    if(include_reverse){
        window.location.href="/img/pixelapp/render/" + window.pixel_id + "/" + val + "/both/" + angle + "/" + flip;
    }else{
        window.location.href="/img/pixelapp/render/" + window.pixel_id + "/" + val + "/forward/" + angle + "/" + flip;
    }
    
}

function getFileDownload(){
    // http://localhost:8081/api/pixelapp/5b7f7738-101e-493b-a427-fdf88f2f29fd
    let url = "/api/pixelapp/download/" + window.pixel_id;
    let filename = $('#filename').val();
    $('#filenameerror').html("");
    if(filename == undefined || filename == null || filename == ""){
        $('#filenameerror').html("Filename cannot be blank");
        return;
    }
    filename = filename + ".json";
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        beforeSend: function (xhr) {
            xhr.setRequestHeader ("Authorization", "Bearer " + token);
        },
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

function getSubColorSpritesheet(){
    let query = "?"
    let bFoundOne = false;
    // Get the color substitutes
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
            window.location.href='/img/pixelapp/' + window.pixel_id  + "/spritesheet" + query;
        }
        
    })
}