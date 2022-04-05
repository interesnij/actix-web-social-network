on('#ajax', 'click', '.track_edit', function() {
  parent = this.parentElement.parentElement.parentElement;
  blocks = document.body.querySelectorAll('.col-sm-12');
  for (var i = 0; i < blocks.length; i++) {blocks[i].classList.remove("edited_track")}

  parent.parentElement.parentElement.parentElement.classList.add("edited_track")
  create_fullscreen("/music/edit_track/" + parent.getAttribute("data-pk") +"/", "item_fullscreen");
});

on('#ajax', 'click', '.load_profile_playlist', function() {
  profile_list_block_load(this, ".load_block", "/music_list/", "load_profile_playlist");
});

on('#ajax', 'click', '.load_attach_playlist', function() {
  profile_list_block_attach(this, ".load_block", "/u_music_list_load/", "load_attach_playlist");
});

on('#ajax', 'click', '.load_music_list', function() {
  if (this.getAttribute("playlist-pk")) {
    playlist_pk = this.getAttribute("playlist-pk");
    owner_pk = null
  } else {
    card = this.parentElement.parentElement.parentElement;
    playlist_pk = card.getAttribute("playlist-pk");
    owner_pk = card.getAttribute("owner-pk");
  };

  create_fullscreen("/music/load_list/" + playlist_pk + "/", "item_fullscreen");
  if (owner_pk) {
    window.history.pushState(null, "vfgffgfgf", window.location.href + "?key=wall&owner_id=" + owner_pk + "&playlist=" + playlist_pk);
  }
});
