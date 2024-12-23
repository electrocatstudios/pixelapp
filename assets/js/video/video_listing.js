$(document).ready(function() {
    var url = "/api/video";
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret){
            if(ret.status != "ok"){
                $('#error').html(ret.message);
                return;
            }
            var output = "";
            for(var i=0;i<ret.videos.length;i++){
                let v = ret.videos[i];
                output += "<div class='row choose_menu_items'>";
                output += "<div class='column choose_menu_item'><p>Name: " + v.name + "</p>";
                output += "<p>Description: " + v.description + "</p>";
                output += "<p>Frames: " + v.frames + "</p>";
                // output += "<button onclick='load_animation(\""+ p.guid + "\")'>Open</button>";
                output += "</div>";
                output += "</div>";
            }
            $('#coll_container').html(output);
        },
        error: function(ret){
            console.log("Error getting saved videos")
            console.log(ret)
        }
    })

});