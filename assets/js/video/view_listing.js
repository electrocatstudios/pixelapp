$(document).ready(function() {
    var url = "/api/view";
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
            for(var i=0;i<ret.views.length;i++){
                let v = ret.views[i];
                output += "<div class='row choose_menu_items'>";
                output += "<div class='column choose_menu_item'><p>Name: " + v.name + "</p>";
                output += "<p>Guid: " + v.guid + "</p>";
                output += "<button onclick='load_view_page(\"" + v.guid + "\")'>Open</button>";
                // output += "<button onclick='delete_animation(\""+ p.guid + "\")'>Delete</button>";
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

function load_view_page(guid) {
    window.location.href='/view_preview/' + guid;
}