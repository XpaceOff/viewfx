<script>
    export let tittle;
    export let value;
    export let min = 10;
   
    function validate(e){
        let userValue = e.target.value;
        userValue = parseInt(userValue.replace(/\D/g, ''));

        if (userValue < min) userValue = min;

        value = userValue;
    }
</script>

<div class="flex flex-row w-full hover:bg-zinc-600 px-2 py-1 rounded-lg text-zinc-500 hover:text-zinc-300 items-center">
    <div class="flex w-8/12">
        {tittle}
    </div>
    <!-- <div class="flex w-4/12 ">
        {value}
    </div> -->
    <input 
        class=" bg-zinc-900 rounded-md border-0 px-2"
        value={value}
        on:change={validate}
    />
</div>