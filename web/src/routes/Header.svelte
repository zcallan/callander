<script>
	import { page } from '$app/stores';
</script>

<header>
	<div class="logo">Callander</div>

	<!-- <nav>
		<ul>
			<li aria-current={$page.url.pathname === '/' ? 'page' : undefined}>
				<a href="/">Home</a>
			</li>
		</ul>
	</nav> -->
</header>

<style>
	header {
		height: 80px;
		display: flex;
		align-items: center;
		padding: 0 2rem;
		background-color: #f8f8f8;
	}

	.logo {
		font-family: var(--font-heading);
		font-size: 1.5rem;
	}
</style>
