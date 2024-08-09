<script lang="ts">
    import FilledHouse from '$env/static/public/icons/house-blank.svg';
    
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
</script>

<nav>
    {#each navOptions as option}
        <a href={option.href}>
            <FilledHouse />
        </a>
    {/each}
</nav>