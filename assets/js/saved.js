var token = null;
$(document).ready(function(){
    // token = getToken();
    // if(token == null){
    //     window.location.href='/login?ret=pixelapp%2fsaved';
    //     return;
    // }
    var url = "/api/pixel";
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        // beforeSend: function (xhr) {
        //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
        // },
        success: function(ret){
            // console.log(ret);
            if(ret.status != "ok"){
                $('#error').html(ret.message);
                return;
            }
            var output = "";
            for(var i=0;i<ret.pixelimages.length;i++){
                var p = ret.pixelimages[i];
                output += "<div class='row choose_menu_items'>"
                // output += "<a href='/pixelapp/" + p.id + "'>"
                output += "<div class='column choose_menu_item'>" + p.name + "<br>" + p.description + "<br>";
                output += "<br><img src='/img/" +p.guid+"/png' height='100'><br>"
                output += "<button onclick='load_image(\""+ p.guid + "\")'>Open</button>";
                output += "<button onclick='load_render_page(\"" + p.guid + "\")'>Render</button>";
                output += "<button onclick='load_png_image(\"" + p.guid + "\")'>PNG</button>";
                output += "<button onclick='show_delete_confirm(\"" + p.guid + "\")'>Delete</button>";
                output += "<div id='confirm_delete_" + p.guid + "' style='display:none;'>";
                output += "This process is unrecoverable are you sure?<br>"
                output += "<button onclick='delete_pixel(\"" + p.guid + "\")'>Confirm</button>";
                output += "<button onclick='cancle_confirmation(\"" + p.guid + "\")'>Cancel</button>";
                
                output += "</div>"
                output += "</div>";
                // output += "</a>";
                output += "</div>";

            }
            
            $('#saved_items').html(output);
        },
        error: function(ret){
            console.log("Error getting saved pixels")
            console.log(ret)
        }
    })
})

function load_image(id){
    window.location.href='/pixel/' + id;
}

function load_render_page(id){
    window.location.href='/render/' + id;
}

function load_png_image(id){
    window.location.href='/img/' + id + '/png'
    
}

function show_delete_confirm(pid){
    $('#confirm_delete_'+ pid).show();
}
function cancle_confirmation(pid){
    $('#confirm_delete_'+ pid).hide();
}
function delete_pixel(pid){
    var url = "/api/pixel/" + pid;
    $.ajax({
        url: url,
        type: 'DELETE',
        dataType: 'json',
        // beforeSend: function (xhr) {
        //     xhr.setRequestHeader ("Authorization", "Bearer " + token);
        // },
        success: function(ret){
            // console.log(ret);
            if(ret.status != "ok"){
                console.log(ret.message);
                $('#error').html(ret.message);
                return;
            }
    
            window.location.reload();
          
        },
        error: function(ret){
            console.log("ERROR while deleting saved data");
            console.log(ret);
        }
      })
}