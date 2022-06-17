on('#ajax', 'click', '.track_edit', function() {
  parent = this.parentElement.parentElement.parentElement;
  blocks = document.body.querySelectorAll('.col-sm-12');
  for (var i = 0; i < blocks.length; i++) {blocks[i].classList.remove("edited_track")}

  parent.parentElement.parentElement.parentElement.classList.add("edited_track")
  create_fullscreen("/music/edit_track/" + parent.getAttribute("data-pk") +"/", "item_fullscreen");
});

on('#ajax', 'click', '.load_profile_music_list', function() {
  profile_list_block_load(this, ".load_block", "/music_list/", "load_profile_music_list");
});

on('#ajax', 'click', '.load_attach_playlist', function() {
  profile_list_block_attach(this, ".load_block", "/u_music_list_load/", "load_attach_playlist");
});

on('#ajax', 'click', '.load_music_list', function() {
  parent = this.parentElement.parentElement.parentElement;
  if (parent.getAttribute("owner-pk")) {
    doclist_pk = parent.getAttribute("playlist-pk");
    owner_pk = parent.getAttribute("owner-pk");
  }
  else {
    doclist_pk = parent.getAttribute("playlist-pk");
    owner_pk = null;
  };

  create_fullscreen("/music/load_list/" + playlist_pk + "/", "item_fullscreen");
  if (owner_pk) {
    window.history.pushState(null, "vfgffgfgf", window.location.href + "?key=wall&owner_id=" + owner_pk + "&playlist=" + playlist_pk);
  }
});
