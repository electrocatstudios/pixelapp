$(document).ready(function(){
    fetch_avaialable_views();
})

function create_animation(){
    $('#error').html("");

    var name = $('#name').val();
    var width = $('#imgwidth').val();
    var height = $('#imgheight').val();
    var length = $('#animlength').val();
    
    if(name == undefined || name == null || name == ""){
        $('#error').html("Name cannot be blank");
        return;
    }

    var data = {
        name: name,
        description: "", // Blank for now - maybe later
        width: parseInt(width),
        height: parseInt(height),
        length: parseInt(length),
    }

    var url = '/api/animation_new';
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

function fetch_avaialable_views() {
    var url = '/api/view';
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret){
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }

            if(ret.views.length < 1){
                $('#view_select').html("No views available");
                return;
            }
            var output = "Select a view to use as template:"
            output += "<select id='sel_view' onchange='update_selected_view()'>";
            output += "<option value=''>---Select a View----</option>";
            for(var i=0;i<ret.views.length;i++){
                var view = ret.views[i];
                output += "<option value='" + view.guid + "'>" + view.name + "</option>";
            };
            output += "</select>"
            $('#view_select').html(output);
        },
        error: function(err){
            console.log("Error getting view list");
            console.log(err);
        }
    });
}

function update_selected_view() {
    // Update width and height from details of guid
    var guid = $('#sel_view').val();
    if(guid === undefined || guid === null || guid === "") {
        return;
    }
    var url = "/api/view_dim/" + guid;
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret){
            
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            $('#imgwidth').val(ret.width);
            $('#imgheight').val(ret.height);
            
        },
        error: function(err){
            console.log("Error getting view dimensions");
            console.log(err);
        }
    })
}