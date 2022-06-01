on('#ajax', 'click', '.send_manager_messages', function() {
  create_fullscreen("/managers/send_messages/", "worker_fullscreen");
});

on('#ajax', 'click', '.create_communities_category', function() {
  create_fullscreen("/admin/created/create_communities_category/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_goods_category', function() {
  create_fullscreen("/admin/created/create_goods_category/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_sound_genre', function() {
  create_fullscreen("/admin/created/create_sound_genre/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_artist', function() {
  create_fullscreen("/admin/created/create_artist/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_music_album', function() {
  create_fullscreen("/admin/created/create_music_album/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_post_category', function() {
  create_fullscreen("/admin/created/create_post_category/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_video_category', function() {
  create_fullscreen("/admin/created/create_video_category/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_stickers_category', function() {
  create_fullscreen("/admin/created/create_stickers_category/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_sticker', function() {
  create_fullscreen("/admin/created/create_sticker/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_smiles_category', function() {
  create_fullscreen("/admin/created/create_smiles_category/", "worker_fullscreen");
});

on('#ajax', 'click', '.create_smile', function() {
  create_fullscreen("/admin/created/create_smile/", "worker_fullscreen");
});
on('#ajax', 'click', '.create_reaction', function() {
  create_fullscreen("/admin/created/create_reaction/", "worker_fullscreen");
});

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
