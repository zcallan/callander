<script lang="ts">
  import { page } from '$app/stores';

  const navItems = [
    {
      label: 'Friends',
      href: '/friends',
    },
    {
      label: 'Posts',
      href: '/posts',
    },
  ];
</script>

<header>
  <a href="/">
    <div class="logo">Callander</div>
  </a>

  <nav>
    <ul>
      {#each navItems as item}
        <li aria-current={$page.url.pathname === item.href ? 'page' : undefined}>
          <a href={item.href}>{item.label}</a>
        </li>
      {/each}
    </ul>
  </nav>
</header>

<style>
  header {
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 2rem;
    background-color: var(--color-bg-0);
  }

  .logo {
    font-family: var(--font-heading);
    font-size: 1.5rem;
  }

  ul {
    list-style: none;
    display: flex;
  }

  li[aria-current='page'] a {
    background-color: rgba(0, 0, 0, 0.05);
  }

  li a {
    display: block;
    padding: 16px;
    margin-left: 8px;
    border-radius: 4px;
    transition: 100ms ease background-color;
  }

  li a:hover {
    background-color: rgba(0, 0, 0, 0.1);
  }
</style>
