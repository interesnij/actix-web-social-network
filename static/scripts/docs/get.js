
on('#ajax', 'click', '.load_attach_doc_list', function() {
  profile_list_block_attach(this, ".load_block", "/u_doc_list_load/", "load_attach_doc_list");
});

on('body', 'click', '.doc_edit', function() {
  parent = this.parentElement.parentElement.parentElement;
  blocks = document.body.querySelectorAll('.col-sm-12');
  for (var i = 0; i < blocks.length; i++) {blocks[i].classList.remove("edited_doc")}

  parent.parentElement.parentElement.parentElement.classList.add("edited_doc")
  create_fullscreen("/docs/edit_doc/" + parent.getAttribute("data-pk") +"/", "worker_fullscreen");
});

on('#ajax', 'click', '.load_doc_list', function() {
  parent = this.parentElement.parentElement.parentElement;
  if (parent.getAttribute("owner-pk")) {
    doclist_pk = parent.getAttribute("doclist-pk");
    owner_pk = parent.getAttribute("owner-pk");
  }
  else {
    doclist_pk = parent.getAttribute("doclist-pk");
    owner_pk = null;
  };

  create_fullscreen("/docs/load_list/" + doclist_pk + "/", "item_fullscreen");
  if (owner_pk) {
    window.history.pushState(null, "vfgffgfgf", window.location.href + "?key=wall&owner_id=" + owner_pk + "&doclist=" + doclist_pk);
  }
});
