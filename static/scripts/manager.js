on('#ajax', 'click', '.send_manager_messages', function() {
  create_fullscreen("/managers/send_messages/", "worker_fullscreen");
});

//////////////////////////////////////////////////
on('#ajax', 'click', '.create_communities_category', function() {
  create_fullscreen("/admin/created/create_communities_category/", "item_fullscreen");
});
on('#ajax', 'click', '.create_goods_category', function() {
  create_fullscreen("/admin/created/create_goods_category/", "item_fullscreen");
});
on('#ajax', 'click', '.create_sound_genre', function() {
  create_fullscreen("/admin/created/create_sound_genre/", "item_fullscreen");
});
on('#ajax', 'click', '.create_artist', function() {
  create_fullscreen("/admin/created/create_artist/", "item_fullscreen");
});
on('#ajax', 'click', '.create_music_album', function() {
  create_fullscreen("/admin/created/create_music_album/", "item_fullscreen");
});
on('#ajax', 'click', '.create_post_category', function() {
  create_fullscreen("/admin/created/create_post_category/", "item_fullscreen");
});
on('#ajax', 'click', '.create_video_category', function() {
  create_fullscreen("/admin/created/create_video_category/", "item_fullscreen");
});
on('#ajax', 'click', '.create_stickers_category', function() {
  create_fullscreen("/admin/created/create_stickers_category/", "item_fullscreen");
});
on('#ajax', 'click', '.create_sticker', function() {
  create_fullscreen("/admin/created/create_sticker/", "item_fullscreen");
});
on('#ajax', 'click', '.create_smiles_category', function() {
  create_fullscreen("/admin/created/create_smiles_category/", "item_fullscreen");
});
on('#ajax', 'click', '.create_smile', function() {
  create_fullscreen("/admin/created/create_smile/", "item_fullscreen");
});
on('#ajax', 'click', '.create_reaction', function() {
  create_fullscreen("/admin/created/create_reaction/", "item_fullscreen");
});

on('#ajax', 'click', '.create_communities_category_btn', function() {
  send_form_and_close_window("/admin/created/create_communities_category/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_goods_category', function() {
  send_form_and_close_window("/admin/created/create_goods_category/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_sound_genre_btn', function() {
  send_form_and_close_window("/admin/created/create_sound_genre/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_artist_btn', function() {
  send_form_and_close_window("/admin/created/create_artist/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_music_album_btn', function() {
  send_form_and_close_window("/admin/created/create_music_album/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_post_category_btn', function() {
  send_form_and_close_window("/admin/created/create_post_category/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_video_category_btn', function() {
  send_form_and_close_window("/admin/created/create_video_category/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_stickers_category_btn', function() {
  send_form_and_close_window("/admin/created/create_stickers_category/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_sticker_btn', function() {
  send_form_and_close_window("/admin/created/create_sticker/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_smiles_category_btn', function() {
  send_form_and_close_window("/admin/created/create_smiles_category/", this.parentElement.parentElement.parentElement);
});

on('#ajax', 'click', '.create_smile_btn', function() {
  send_form_and_close_window("/admin/created/create_smile/", this.parentElement.parentElement.parentElement);
});
on('#ajax', 'click', '.create_reaction_btn', function() {
  send_form_and_close_window("/admin/created/create_reaction/", this.parentElement.parentElement.parentElement);
});
///////////////////////////////////////////

on('body', 'click', '.create_close', function() {
  parent = this.parentElement;
  type = parent.getAttribute('data-type');
  if (parent.getAttribute('data-subtype')) {
    subtype = parent.getAttribute('data-subtype')
  } else { subtype = null};
  create_fullscreen("/managers/create_sanction/?types=" + type + "&subtype=" + subtype, "worker_fullscreen");
});

on('body', 'click', '.submit_case_sanction', function() {
  if (this.classList.contains('submit_case_suspend')) {
    this.parentElement.parentElement.querySelector('.block_suspend').classList.remove('hidden')
  } else { this.parentElement.parentElement.querySelector('.block_suspend').classList.add('hidden')};
});

on('#ajax', 'click', '#send_manager_messages_btn', function() {
  form = this.parentElement.parentElement.parentElement;
  _text = form.querySelector(".smile_supported").innerHTML;
  if (_text.replace(/<[^>]*(>|$)|&nbsp;|&zwnj;|&raquo;|&laquo;|&gt;/g,'').trim() == "" && form.querySelector(".files_0")){
    toast_error("Напишите или прикрепите что-нибудь");
    return
  } else {
    this.disabled = true;
  };
  $input = document.createElement("input");
  $input.setAttribute("name", "content");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text;
  form.append($input);
  form_data = new FormData(form);

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'POST', '/managers/send_messages/', true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            toast_success("Рассылка создана");
            close_work_fullscreen();
        } else {this.disabled = false}
      }
      ajax_link.send(form_data);
});


on('#ajax', 'click', '#create_sanction_btn', function() {
  form = this.parentElement.parentElement;
  _text = form.querySelector(".smile_supported").innerHTML;
  this.disabled = true;

  $input = document.createElement("input");
  $input.setAttribute("name", "description");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text;
  form.append($input);
  form_data = new FormData(form);

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'POST', '/managers/create_sanction/', true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            toast_success("Санкция применена!");
            close_work_fullscreen();
        } else {this.disabled = false}
      }
      ajax_link.send(form_data);
});
