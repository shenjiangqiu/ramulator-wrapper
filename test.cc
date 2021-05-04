#include "dramsim2_wrapper.h"
#include <iostream>
int main()
{
    auto dram = get_dramsim2();
    for (int i = 0; i < 10; i++)
    {
        dramsim2_send(dram, i * 64, false);
        dramsim2_tick(dram);
    }
    for (int i = 0; i < 10; i++)
    {
        while (!dramsim2_ret_available(dram))
        {
            dramsim2_tick(dram);
        }
        auto ret = dramsim2_get(dram);
        std::cout << ret << std::endl;
    }
}