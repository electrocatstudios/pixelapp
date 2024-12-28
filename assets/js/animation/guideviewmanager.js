$(document).ready(function() {
    if(window.view_guid === undefined || window.view_guid === null || window.view_guid === ""){
        // Do nothing there is no view attached
        return;
    }

    // Load images from view
    var url = "/api/view/" + window.view_guid;
    $.ajax({
        url: url,
        type: 'GET',
        dataType: 'json',
        success: function(ret){
            if(ret.status != 'ok'){
                $('#error').html(ret.message);
                return;
            }
            console.log(ret);
            // TODO: create images and attach them to the global guide view manager
            // Also probably consider putting this somewhere more sensible, like
            // the manager initiation code.
            // window.location.href='/';
        },
        error: function(ret){
            console.log("ERROR creating new collection");
            console.log(ret);
        }
    })
});

function GuideViewManager() {
    this.update = GuideViewManagerUpdate;
    this.draw = GuideViewManagerDraw;
}

function GuideViewManagerUpdate() {
    // Do updates - calculate current frame showing
}

function GuideViewManagerDraw(ctx) {

}