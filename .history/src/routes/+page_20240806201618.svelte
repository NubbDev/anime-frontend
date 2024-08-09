<script lang="ts">
 //http://localhost:1420
</script>

<main>
  <h1>Page</h1>
  <p>This is a page.</p>

  <nav>
    <ul>
      <li><a href="/">H</a></li>
      <li><a href="/search">S</a></li>
      <li><a href="/profile">P</a></li>
    </ul>
  </nav>
</main>

<style>
  nav {
    display: flex;
    justify-content: center;
    position: fixed;
    bottom: 0;
    left: 0;
    margin: 0;
    width: 100vw;
    background-color: black;
  }

  nav ul {
    display: flex;
    justify-content: space-between;
    list-style-type: none;
    padding: 0;
  }
</style>