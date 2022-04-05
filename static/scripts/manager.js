on('#ajax', 'click', '.send_manager_messages', function() {
  create_fullscreen("/managers/send_messages/", "worker_fullscreen");
});

on('body', 'click', '.create_close', function() {
  parent = this.parentElement;
  type = parent.getAttribute('data-type');
  if (parent.getAttribute('data-subtype')) {
    subtype = parent.getAttribute('data-subtype')
  } else { subtype = null};
  create_fullscreen("/managers/create_sanction/?type=" + type + "&subtype=" + subtype, "worker_fullscreen");
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
  $input.setAttribute("name", "text");
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
