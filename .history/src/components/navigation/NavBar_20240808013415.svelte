<script lang="ts">
    import FilledHouse from '$lib/assets/icons/house-blank-filled.svg';
  import { onMount } from 'svelte';
    // assets/icons/house-blank.svg
    const navOptions: { name: string, href: string, selected: string, unSelected: string }[] = [
        {
            name: 'Home',
            href: '/',
            selected: 'house-blank-filled',
            unSelected: 'house-blank'
        },
        {
            name: 'Releases',
            href: '/releases',
            selected: 'calendar-solid',
            unSelected: 'calendar-thin'
            
        },
        {
            name: 'Downloads',
            href: '/downloads',
            selected: 'inbox-in-filled',
            unSelected: 'inbox-in'
        },
        {
            name: 'Profile',
            href: '/profile',
            selected: 'circle-user-filled',
            unSelected: 'circle-user'
        }
    ]

    let path: string;
    onMount(() => {
        path = '/' + window.location.pathname.split('/')[1];
        console.log(path);
    })
</script>

<nav>
    
    {#each navOptions as option}
        <a href={option.href}>
            {#if path === option.href} 
                <img src={FilledHouse} alt={option.name} style="fill: white;">
                <span>{option.name}</span>
            {:else}
                <img src={FilledHouse} alt={option.name} >
                <span style="color: var(--icon-unselected);">{option.name}</span>
            {/if}
        </a>
    {/each}
</nav>

<style>
    nav {
        display: flex;
        justify-content: space-between;
        box-sizing: border-box;
        padding: 0 10vw;
        position: fixed;
        bottom: 0;
        left: 0;
        margin: 0;
        width: 100vw;
        background-color: rgba(18, 18, 18, 0.8);
        backdrop-filter: 16;
    }

    nav a img {
        width: 35px;
        height: 35px;
    }
    nav a span {
        color: white;
        font-size: 0.6rem;
        text-decoration: none;
    }
    nav a {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-decoration: none;
    }
</style>