on('#ajax', 'click', '.load_profile_good_list', function() {
  profile_list_block_load(this, ".load_block", "/goods_list/", "load_profile_good_list");
});
on('#ajax', 'click', '.load_attach_good_list', function() {
  profile_list_block_attach(this, ".load_block", "/u_good_list_load/", "load_attach_good_list");
});

on('#ajax', 'click', '.good_detail', function() {
  pk = this.getAttribute('good-pk');
  create_fullscreen('/goods/good/' + pk + '/', "item_fullscreen");
  container = document.body.querySelector("#fullscreens_container");
  loader = container.querySelector(".card_fullscreen");
  setTimeout(function() {good_gallery(loader)}, 1000)
});

on('#ajax', 'click', '.load_good_list', function() {
  if (this.getAttribute("goodlist-pk")) {
    goodlist_pk = this.getAttribute("goodlist-pk");
    owner_pk = null
  } else {
    card = this.parentElement.parentElement.parentElement;
    goodlist_pk = card.getAttribute("goodlist-pk");
    owner_pk = card.getAttribute("owner-pk");
  };
  create_fullscreen("/goods/load_list/" + goodlist_pk + "/", "item_fullscreen");
  if (owner_pk) {
    window.history.pushState(null, "vfgffgfgf", window.location.href + "?key=wall&owner_id=" + owner_pk + "&goodlist=" + goodlist_pk);
  }
});
