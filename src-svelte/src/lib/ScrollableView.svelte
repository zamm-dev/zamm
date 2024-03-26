<script lang="ts">
  import Scrollable from "./Scrollable.svelte";
</script>

<div class="container">
  <Scrollable {...$$restProps}>
    <p>
      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque faucibus
      vulputate rhoncus. Maecenas velit urna, consectetur id ipsum ac, porta
      accumsan neque. Vestibulum sit amet magna aliquet, posuere lorem eu,
      aliquet sapien. Duis ac arcu a lacus dictum hendrerit. Integer vel lorem
      dapibus, interdum erat eget, iaculis sem. Mauris massa turpis, vulputate
      ut iaculis nec, aliquet id massa. Aenean nunc tortor, viverra sed auctor
      ut, viverra vel dolor. Duis molestie maximus felis id lacinia.
      Pellentesque venenatis diam sit amet posuere tincidunt. In hac habitasse
      platea dictumst. Pellentesque in mattis ipsum. Donec tempus mi eu varius
      egestas. Nulla mattis metus lectus, eget auctor risus aliquam vitae.
      Aenean ullamcorper elementum enim, vitae viverra mi dignissim sit amet.
      Cras semper enim eget sapien gravida, eget laoreet elit lobortis. Nam
      dictum, dolor sit amet rhoncus tincidunt, quam lorem faucibus nunc, sed
      convallis orci sapien dignissim nisi.
    </p>

    <p>
      Donec eu hendrerit mauris, hendrerit iaculis nulla. Nam venenatis volutpat
      eleifend. Donec quis dolor at arcu commodo aliquet. Duis feugiat nisi in
      ex viverra, eu facilisis ante tempus. Cras sed porta ipsum. Donec dui
      diam, ultrices eget neque feugiat, malesuada cursus arcu. In eget laoreet
      ligula, eget finibus quam. Duis congue porta libero, vitae porttitor erat.
      Suspendisse ipsum dui, condimentum vitae dictum sed, feugiat non leo.
      Integer tristique sollicitudin ullamcorper. Nullam elementum facilisis
      nulla vitae congue. In cursus est commodo ante mollis pharetra.
    </p>

    <p>
      Integer at nunc suscipit sem molestie facilisis. Morbi iaculis bibendum
      dui, quis convallis mauris bibendum ut. Suspendisse arcu nunc, tristique
      sit amet nisi ut, varius lacinia orci. Nulla tincidunt orci id tellus
      ultricies, eu vestibulum neque tincidunt. Nam at nisi justo. Etiam lacinia
      aliquet sem, sit amet facilisis erat vestibulum quis. Proin ut semper
      turpis, vel euismod nibh. Ut in tempor risus. Vivamus ullamcorper molestie
      laoreet. Maecenas id fermentum tellus. Curabitur dui leo, porta id tortor
      non, dignissim vehicula lectus. Nulla et diam at metus accumsan varius sit
      amet id sapien.
    </p>

    <p>
      Pellentesque lacinia enim ut commodo cursus. Nullam rhoncus pretium
      ornare. Donec pulvinar urna vel tempor accumsan. Nunc viverra purus non
      lacus condimentum, ut sodales orci placerat. Vivamus luctus dictum nisi.
      Donec turpis diam, dictum eget ex non, vulputate molestie dui. Duis urna
      lectus, tempor in bibendum venenatis, congue eu tortor. Duis at dictum
      sapien, bibendum porta magna. Quisque nisi eros, vulputate nec dui ut,
      scelerisque rhoncus ante. In varius metus mi, in elementum lorem venenatis
      lacinia. Proin risus lacus, ultrices eget dolor sed, sodales ullamcorper
      orci. Vivamus non augue pulvinar, congue orci nec, imperdiet neque. Cras
      ligula nisl, pharetra vel libero vitae, efficitur lacinia nunc. Maecenas
      facilisis tortor a arcu egestas commodo.
    </p>

    <p>
      Nunc ac pellentesque ex, sit amet posuere purus. In ac tempor libero, vel
      pretium metus. Nam fermentum ut arcu vehicula fermentum. Mauris euismod
      magna vitae ipsum aliquam tristique. Duis luctus arcu dictum nibh egestas,
      eu fringilla augue ultricies. Fusce in augue consequat, luctus nisl vitae,
      sollicitudin mauris. In ultrices augue sit amet nisi luctus imperdiet. In
      non felis euismod, viverra metus id, hendrerit leo. Curabitur tristique
      erat et nisl mattis, eu tempus nisl tristique. Praesent vel sem
      pellentesque libero sodales lobortis nec a neque. Mauris at lorem auctor,
      tincidunt augue nec, lobortis sem.
    </p>
  </Scrollable>
  <div class="other-element">
    <p>
      This element takes up fixed space. The Scrollable above should expand to
      fill the rest of the space.
    </p>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    height: calc(100vh - 2rem);
  }

  .other-element {
    background-color: #f0f0f0;
    padding: 1rem;
  }
</style>
