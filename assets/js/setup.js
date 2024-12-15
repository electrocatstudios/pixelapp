var animation_list = [];

$(document).ready(function(){
    var url = "/api/collection";
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        // beforeSend: function (xhr) {
        //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
        // },
        success: function(ret){
            if(ret.status != "ok"){
                $('#error').html(ret.message);
                return;
            }
            var output = "<select id='sel_collection'><option value='0'>--No Collection--</option>";
            for(var i=0;i<ret.collections.length;i++){
                var c = ret.collections[i];
                if(c.name === ""){
                    output += "<option value='" + c.id + "'>[None]</option>"
                } else {
                    output += "<option value='" + c.id + "'>" + c.name + "</option>"
                }

            }
            output += "</select>";

            $('#coll_container').html(output);
        },
        error: function(ret){
            console.log("Error getting saved pixels")
            console.log(ret)
        }
    });

    update_animation_list();
})

function create_picture(){
    $('#error').html("");

    var name = $('#name').val();
    var desc = $('#description').val();
    var width = $('#imgwidth').val();
    var height = $('#imgheight').val();
    var pixelwidth = $('#pixelwidth').val();
    var collection = $('#sel_collection').val();
    if(collection === 0) {
        collection = null;
    }

    if(name == undefined || name == null || name == ""){
        $('#error').html("Name cannot be blank");
        return;
    }

    var anim_sel = $('#selected_animation').val();
    if(anim_sel === "") {
        anim_sel = null;
        var frame_count = $('#frame_count').val();
        try {
            frame_count = parseInt(frame_count);            
        } catch {
            frame_count = null;
        }
    }

    var data = {
        name: name,
        description: desc,
        collection: parseInt(collection),
        width: parseInt(width),
        height: parseInt(height),
        pixelwidth: parseInt(pixelwidth),
        animation: anim_sel,
        frame_count: frame_count
    }

    // // console.log(data);
    // if(token == null || token.trim() == ""){
    //     $('#error').html("No token available")
    //     window.location.href='/login';
    //     return;
    // }

    var url = '/api/new';
    $.ajax({
        url: url,
        type: 'POST',
        dataType: 'json',
        contentType: "application/json; charset=utf-8",
        data: JSON.stringify(data),
        // beforeSend: function (xhr) {
        //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
        // },
        success: function(ret){
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            window.location.href='/pixel/' + ret.pixelid;
        },
        error: function(ret){
            $('#error').html("Error creating new pixel");
            console.log("ERROR while creting new pixel");
            console.log(ret);
        }
    })
}

function load_from_file(){
    let filename = $("#filename")[0].files[0];
    let reader = new FileReader();
    
    var name = $('#name').val();
    var desc = $('#description').val();

    // Closure to capture the file information.
    reader.onload = (function(filename) {
      return function(e) {
        let res = JSON.parse(e.target.result);
        res.name = name;
        res.description = desc;
        
        let url = "/api/newfromfile";
        console.log(res);
        $.ajax({
            url: url,
            type: 'POST',
            data: JSON.stringify(res),
            dataType: 'json',
            contentType: "application/json; charset=utf-8",
            // beforeSend: function (xhr) {
            //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
            // },
            success: function(ret){
                if(ret.status != 'ok'){
                    $('#errorfileupload').html(ret.message);
                    return;
                }

                window.location.href='/pixel/' + ret.guid;
            },
            error: function(ret){
                console.log("ERROR while creting new game");
                console.log(ret);
            }
        })
      };
    })(filename);
    
    reader.readAsText(filename);
}

// Push details of existing animations 
function update_animation_list() {
    animation_list = [];

    $.ajax({
        url: "api/animation",
        type: 'GET',
        dataType: 'json',
        contentType: "application/json; charset=utf-8",
        success: function(ret) {
            if(ret.status !== 'ok') {
                $("#error").html(ret.message);
                return;
            }
            var output = "Animation: <select id='selected_animation' onchange='animation_select_changed()'>";
            output += "<option value=''>--Select an Animation--</option>";
            for(var i=0;i<ret.animations.length;i++){
                var a = ret.animations[i];
                output += "<option value='" + a.guid + "'>" + a.name + "</option>";
            }
            output += "</select>";
            output += "<div id='animation_frame_count'>Frame count: <input id='frame_count' type='number'>";
            $('#animation_selector').html(output);
            $('#animation_frame_count').hide();
            animation_list = ret.animations;
        },
        error: function(err) {
            console.log("ERROR getting animation name list");
            console.log(err);
        }
    });
}


function animation_select_changed() {
    console.log("animation selection changed");
    var sel = $('#selected_animation').val();
    // console.log(sel);
    var bfound = false;
    for(var i=0;i<animation_list.length;i++){
        var a = animation_list[i];
        if(a.guid === sel) {
            $('#imgwidth').val(a.width);
            $('#imgheight').val(a.height);
            bfound = true;
        }
    }

    if(!bfound) {
        console.log("Was unable to find the selected animation")
    }

    if(sel === "") {
        $('#animation_frame_count').hide();
    } else {
        $('#animation_frame_count').show();
    }
}