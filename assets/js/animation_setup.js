$(document).ready(function(){
    
})

function create_animation(){
    $('#error').html("");

    var name = $('#name').val();
    var width = $('#imgwidth').val();
    var height = $('#imgheight').val();
    
    if(name == undefined || name == null || name == ""){
        $('#error').html("Name cannot be blank");
        return;
    }

    var data = {
        name: name,
        width: parseInt(width),
        height: parseInt(height),
    }

    var url = '/api/new_animation';
    $.ajax({
        url: url,
        type: 'POST',
        dataType: 'json',
        contentType: "application/json; charset=utf-8",
        data: JSON.stringify(data),
        success: function(ret){
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            window.location.href='/animation/' + ret.animationid;
        },
        error: function(ret){
            $('#error').html("Error creating new pixel");
            console.log("ERROR while creting new pixel");
            console.log(ret);
        }
    })
}
