<script type="text/javascript">
  function InitCopyPaste(){
    const codeBlocks = document.querySelectorAll("div.highlighter-rouge");

    codeBlocks.forEach((codeblock, index) => {
      const code = codeBlocks[index].innerText;
      const copyCodeButton = document.createElement("button");
      copyCodeButton.innerHTML = "COPY";
      copyCodeButton.classList = "btn btn-sm btn-outline-primary";
      // insert a copy button
      copyCodeButton.onclick = function () {
        window.navigator.clipboard.writeText(code);
        copyCodeButton.innerHTML = "COPIED";
        copyCodeButton.classList.add("btn-success");
        copyCodeButton.classList.remove("btn-outline-primary");

        setTimeout(() => {
          copyCodeButton.innerHTML = "COPY";
          copyCodeButton.classList.remove("btn-success");
          copyCodeButton.classList.add("btn-outline-primary");
        }, 2000);
      };
      // make the button
      codeblock.appendChild(copyCodeButton);
    });
  }

  document.addEventListener("DOMContentLoaded", InitCopyPaste);
</script>