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
    })
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

    var data = {
        name: name,
        description: desc,
        collection: parseInt(collection),
        width: parseInt(width),
        height: parseInt(height),
        pixelwidth: parseInt(pixelwidth)
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