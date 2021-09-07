let token = null;
$(document).ready(function(){
    token = getToken();
    if(token == null){
        window.location.href='/login?ret=pixelapp%2fnew'
        return;
    }
})

function create_picture(){
    $('#error').html("");

    var name = $('#name').val();
    var desc = $('#description').val();
    var width = $('#imgwidth').val();
    var height = $('#imgheight').val();
    var pixelwidth = $('#pixelwidth').val();
    
    if(name == undefined || name == null || name == ""){
        $('#error').html("Name cannot be blank");
        return;
    }

    var data = {
        name: name,
        description: desc,
        width: parseInt(width),
        height: parseInt(height),
        pixelwidth: parseInt(pixelwidth)
    }
    // console.log(data);
    if(token == null || token.trim() == ""){
        $('#error').html("No token available")
        window.location.href='/login';
        return;
    }
    var url = '/api/pixelapp/new';
    // console.log(url);
    $.ajax({
        url: url,
        type: 'POST',
        dataType: 'json',
        data: JSON.stringify(data),
        beforeSend: function (xhr) {
            xhr.setRequestHeader ("Authorization", "Bearer " + token);
        },
        success: function(ret){
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            window.location.href='/pixelapp/' + ret.pixelid;
        },
        error: function(ret){
            console.log("ERROR while creting new game");
            console.log(ret);
        }
    })
}

function load_from_file(){
    let filename = $("#filename")[0].files[0];
    let reader = new FileReader();

    // Closure to capture the file information.
    reader.onload = (function(filename) {
      return function(e) {
        let res = JSON.parse(e.target.result);
        let url = "/api/pixelapp/newfromfile";
        $.ajax({
            url: url,
            type: 'POST',
            data: JSON.stringify(res),
            dataType: 'json',
            beforeSend: function (xhr) {
                xhr.setRequestHeader ("Authorization", "Bearer " + token);
            },
            success: function(ret){
                if(ret.status != 'ok'){
                    $('#errorfileupload').html(ret.message);
                    return;
                }
                window.location.href='/pixelapp/' + ret.pixelid;
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